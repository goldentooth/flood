# Flood
A little Rust app for sending a bunch of HTTP requests as fast as possible.

## Why?

You can get the full background at [Flags](https://github.com/goldentooth/flags/) (I don't have time to repeat it lol), but the **TL;DR**: is that I want to be able to make some very specific HTTP requests at very high frequencies to apps running on a very underpowered cluster, and weirdly, I think that the fastest way to accomplish that might actually be just to write that thing myself.

So let's see how this goes.

## Cross-Compilation

To cross-compile for a Raspberry Pi:

```bash
export DOCKER_DEFAULT_PLATFORM=linux/x86_64/v2
cross build --release --target=aarch64-unknown-linux-gnu
```