
mod html;
mod ui;
mod routes;

#[tokio::main]
async fn main() {
    let app = routes::routes();

    let addr = ([127, 0, 0, 1], 8192).into();
    let server = axum_server::bind(addr);
    println!("serving on http://{addr}");
    server
        .serve(app.into_make_service())
        .await
        .unwrap();
}