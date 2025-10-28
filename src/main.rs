use std::net::SocketAddr;

mod config;
mod db;

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


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_collection_name_via_main() {
        // Ensure the config module is compiled and the helper works
        env::remove_var("MONGO_PRODUCTS_COLLECTION");
        let name = crate::config::database::MongoDB::get_collection_name("MONGO_PRODUCTS_COLLECTION");
        assert_eq!(name, "products".to_string());
    }

    #[tokio::test]
    async fn test_init_connects_if_enabled_via_main() {
        if env::var("RUN_DB_INTEGRATION_TESTS").is_err() {
            eprintln!("skipping integration test; set RUN_DB_INTEGRATION_TESTS=1 to enable");
            return;
        }

        env::set_var("DATABASE_URL", env::var("DATABASE_URL").unwrap_or_else(|_| "mongodb://127.0.0.1:27017".to_string()));
        env::set_var("MONGO_DATABASE", env::var("MONGO_DATABASE").unwrap_or_else(|_| "test_ecommerce_db".to_string()));

        let res = crate::config::database::MongoDB::init().await;
        assert!(res.is_ok());
    }
}