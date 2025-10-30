use crate::db::AppState;
use crate::handlers::{auth as auth_handlers, product as product_handlers};
use crate::middleware::auth::auth_middleware;
use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::env;
use std::sync::Arc;

pub fn create_routes(state: Arc<AppState>) -> Router {
    let jwt_secret = Arc::new(env::var("JWT_SECRET").expect("JWT_SECRET must be set"));

    // Public routes (no authentication)
    let public_routes = Router::new()
        .route("/auth/register", post(auth_handlers::register))
        .route("/auth/login", post(auth_handlers::login))
        .route("/products", get(product_handlers::list_products))
        .route("/products/search", get(product_handlers::search_products))
        .route("/products/{id}", get(product_handlers::get_product));

    // Admin routes (require authentication)
    let admin_routes = Router::new()
        .route("/admin/products", post(product_handlers::create_product))
        .route("/admin/products/{id}", put(product_handlers::update_product))
        .route("/admin/products/{id}", delete(product_handlers::delete_product))
        .layer(middleware::from_fn_with_state(jwt_secret.clone(), auth_middleware));

    // Combine routes
    Router::new()
        .nest("/api", public_routes)
        .nest("/api", admin_routes)
        .with_state(state)
}
