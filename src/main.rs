use std::sync::Arc;
use tiny_http::Server;
use api::app;

mod html;
mod api;
mod hscript;

#[tokio::main]
async fn main() {
    let server = Server::http("0.0.0.0:8090").unwrap();
    let api = Arc::new(app());

    tokio::task::spawn(async move {
        for request in server.incoming_requests() {
            let method = request.method().clone();
            let url = request.url().to_string();
            println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                     method,
                     url,
                     request.headers());

            let api = api.clone();
            tokio::task::spawn(async move {
                let request = request;
                let _ = api.handle(request)
                    .map_err(|e| {
                        eprintln!("error with request! {:?} {:?} | error: {:?}",
                                  method, url, e);
                    });
            });
        }
    });
}
