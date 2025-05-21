use crate::{mdns::browser, shutdown::manager::ShutdownManager};
use derivative::Derivative;
use futures::future::BoxFuture;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use reqwest::Client;
use tokio_util::sync::CancellationToken;

pub type ShutdownTaskReturn = BoxFuture<'static, eyre::Result<()>>;
pub type ShutdownTaskFn =
  dyn FnOnce(CancellationToken, ShutdownContainer) -> ShutdownTaskReturn + Send;
pub type ShutdownTask = Box<ShutdownTaskFn>;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct ShutdownContainer {
  #[derivative(Debug = "ignore")]
  pub service_daemon: ServiceDaemon,
  pub domain: String,
  pub service_info: ServiceInfo,
  pub http_client: Client,
}

impl ShutdownContainer {
  pub fn new(
    service_daemon: ServiceDaemon,
    domain: String,
    service_info: ServiceInfo,
    http_client: Client,
  ) -> Self {
    Self {
      service_daemon,
      domain,
      service_info,
      http_client,
    }
  }

  pub async fn register_tasks(
    &self,
    shutdown: &ShutdownManager,
    _listener: tokio::net::TcpListener,
  ) {
    let tasks: Vec<(&'static str, ShutdownTask)> = vec![
      (
        "browse_services",
        Box::new(|cancel, container| {
          Box::pin(async move { browser::browse_loop(&container, cancel).await })
        }),
      ),
      // (
      //   "gossip_listener",
      //   Box::new(move |cancel, container| {
      //     Box::pin(async move { listener::gossip_listen(&container, listener, cancel).await })
      //   }),
      // ),
    ];

    for (name, task) in tasks {
      self.spawn(shutdown, name, task).await;
    }
  }

  pub async fn spawn<F, Fut>(&self, shutdown: &ShutdownManager, name: &'static str, f: F)
  where
    F: FnOnce(CancellationToken, ShutdownContainer) -> Fut + Send + 'static,
    Fut: Future<Output = eyre::Result<()>> + Send + 'static,
  {
    shutdown.spawn_guarded(name, self, f).await;
  }
}
