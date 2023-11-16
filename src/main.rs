use std::future::Future;
use std::io::{stdout, Write};
use std::net::{SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use anyhow::{anyhow, Context};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::watch;
use tracing::Level;

use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use api::app;
use crate::api::App;

mod html;
mod api;
mod hscript;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_writer(std::io::stderr
                    .with_max_level(Level::TRACE)))
        .init();

    let api = Arc::new(app());
    let (send_shutdown, recv_shutdown) = watch::channel(false);

    tokio::spawn(async move {
        println!("hi!!");
        let worker = server(api.clone(), recv_shutdown).await;
        match worker {
            Ok(worker) => worker.await,
            Err(e) => eprintln!("failed to start server: {}", e),
        };
    });

    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut line = String::new();
    loop {
        {
            let mut stdlock = stdout().lock();
            stdlock.write_all(b"> ").ok();
            stdlock.flush().ok();
        }

        line.clear();
        let _ = stdin.read_line(&mut line).await;
        let line = line.trim();

        if line == "exit" {
            send_shutdown.send(true).ok();
            break;
        }

        println!("received input: {:?}", line);
    }

    Ok(())
}

async fn server(
    api: Arc<App>,
    shutdown_flag: watch::Receiver<bool>,
) -> anyhow::Result<impl Future<Output=()>> {
    let addr = SocketAddr::from_str("0.0.0.0:8090")
        .map_err(|e| anyhow!("failed to parse address: {}", e))?;
    let listener = TcpListener::bind(&addr).await
        .with_context(|| format!("failed to bind address {}", addr))?;
    println!("listening on {}", addr);

    let future = async move {
        println!("start-up");
        let mut shutdown_flag = shutdown_flag;
        loop {
            let conn = select! {
                r = listener.accept() => r,
                _ = shutdown_flag.changed() => break,
            };
            let (stream, addr) = match conn {
                Ok((stream, addr)) => (stream, addr),
                Err(e) => {
                    eprintln!("failed to accept connection: {}", e);
                    continue;
                }
            };

            let h2 = h2::server::handshake(stream).await;
            let mut h2 = match h2 {
                Ok(h2) => h2,
                Err(e) => {
                    eprintln!("failed to perform HTTP/2 handshake: {}", e);
                    continue;
                }
            };

            while let Some(request) = h2.accept().await {
                let Ok((request, response)) = request else {
                    eprintln!("failed to accept request: {}", addr);
                    continue;
                };
                println!("received request: {:?}", request);

                let api = api.clone();
                tokio::spawn(async move {
                    let result = api.handle(request, response).await;
                    if let Err(e) = result {
                        eprintln!("failed to handle request: {}", e);
                    }
                });
            }
        }
    };

    Ok(future)
}