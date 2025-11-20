use std::sync::Arc;
use tokio::net::TcpListener;

use crate::args::Args;

pub struct Server {
    args: Arc<Args>,
}

impl Server {
    pub fn new(args: Arc<Args>) -> Self {
        Server { args }
    }

    pub async fn start(&self) {
        match TcpListener::bind(format!("{}:{}", self.args.bind, self.args.port)).await {
            Ok(listener) => {
                log::info!("Server initialized");
                log::info!("Ready to accept connections");
                loop {
                    match listener.accept().await {
                        Ok((stream, _address)) => {}
                        Err(e) => {
                            log::error!("Failed to accept connection: {}", e);
                        }
                    }
                }
            }
            Err(_e) => {
                log::error!(
                    "Failed to bind to address {}:{}",
                    self.args.bind,
                    self.args.port
                );
                std::process::exit(-1);
            }
        }
    }
}
