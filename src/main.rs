use std::future::Future;
use std::io::{stdout, Write};
use std::net::{SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use anyhow::{anyhow, Context};
use http::Response;
use hyper::server::conn::http1;
use hyper::service::service_fn;
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

use hyper_util::rt::TokioIo;

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
    mut shutdown_flag: watch::Receiver<bool>,
) -> anyhow::Result<impl Future<Output=()>> {
    let addr = SocketAddr::from_str("0.0.0.0:8090")
        .map_err(|e| anyhow!("failed to parse address: {}", e))?;
    let listener = TcpListener::bind(&addr).await
        .with_context(|| format!("failed to bind address {}", addr))?;
    println!("listening on {}", addr);


    let future = async move {
        loop {
            let result = select! {
                _ = shutdown_flag.changed() => break,
                result = listener.accept() => result,
            };

            let (conn, _addr) = match result {
                Ok((conn, addr)) => (conn, addr),
                Err(e) => {
                    eprintln!("failed to accept connection: {}", e);
                    continue;
                }
            };

            let io = TokioIo::new(conn);
            let api = api.clone();
            tokio::task::spawn(async move {
                let builder = http1::Builder::new();
                let service = service_fn(|req| async {
                    api.handle(req).await
                });

                if let Err(e) = builder.serve_connection(io, service).await {
                    eprintln!("failed to serve connection: {}", e);
                }
            });
        }
    };

    Ok(future)
}