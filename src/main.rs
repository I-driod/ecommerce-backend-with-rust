use std::net::SocketAddr;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {

    //Initialize tracing for logging
    tracing_subscriber::fmt::init();


    let app:Router = Router::new().route("/", get(root_handler))
    .route("/health", get(healt_check));

    let addr = SocketAddr::from(([127,0,0,1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();


}


async  fn root_handler() -> &'static str {
    "Welcome to the E-commerve API"
}

async fn healt_check() -> &'static str {
    "OK"
}