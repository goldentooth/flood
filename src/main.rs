use init::args::ArgsStage;
use shutdown::manager::ShutdownManager;
use tokio::signal;
use tracing::{info, instrument};

mod init;
mod log;
mod mdns;
mod node;
mod shutdown;

#[tokio::main]
#[instrument]
async fn main() -> eyre::Result<()> {
  if false {
    log::init()?;
  } else {
    console_subscriber::init();
  }

  let shutdown = ShutdownManager::new();
  let (container, listener) = ArgsStage::parse()?
    .bind_socket()?
    .bind()
    .await?
    .generate_id()
    .build()?
    .finalize();
  container.register_tasks(&shutdown, listener).await;

  shutdown
    .spawn("ctrl_c", {
      let shutdown = shutdown.clone();
      async move {
        signal::ctrl_c().await.expect("failed to listen for event");
        info!("Ctrl-C pressed, shutting down...");
        shutdown.cancel();
      }
    })
    .await;

  shutdown.shutdown().await;
  info!("Shutdown complete");
  Ok(())
}
