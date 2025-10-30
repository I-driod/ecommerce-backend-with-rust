use axum::response::IntoResponse;
use crate::config::database::MongoDB;
use crate::db::AppState;
use crate::middleware::auth::AuthUser;
use crate::models::product::{
    CreateProductRequest, PaginationParams, ProductFilter, UpdateProductRequest,
};
use crate::services::product::ProductService;
use crate::utils::error::{ Result};
use crate::utils::response::ApiResponse;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

// GET /products
pub async fn list_products(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<ProductFilter>,
    Query(pagination): Query<PaginationParams>,
) -> Result<impl IntoResponse> {
    let collection_name = MongoDB::get_collection_name("MONGO_PRODUCTS_COLLECTION");
    let collection = state.collection(&collection_name);

    let products = ProductService::get_products(
        &collection,
        Some(filter),
        None,
        pagination.page,
        pagination.limit,
    )
    .await?;

    let response = ApiResponse::success(serde_json::json!({
        "results": products.len(),
        "data": products
    }));

    Ok(response)
}

// GET /products/search?q=shampoo
pub async fn search_products(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Result<impl IntoResponse> {
    let collection_name = MongoDB::get_collection_name("MONGO_PRODUCTS_COLLECTION");
    let collection = state.collection(&collection_name);

    let products = ProductService::search_products(&collection, &query.q).await?;

    let response = ApiResponse::success(serde_json::json!({
        "results": products.len(),
        "data": products
    }));

    Ok(response)
}

// GET /products/:id
pub async fn get_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let collection_name = MongoDB::get_collection_name("MONGO_PRODUCTS_COLLECTION");
    let collection = state.collection(&collection_name);

    let product = ProductService::get_product_by_id(&collection, &id).await?;

    let response = ApiResponse::success(serde_json::json!({
        "data": product
    }));

    Ok(response)
}

// POST /admin/products (requires authentication)
pub async fn create_product(
    _auth: AuthUser,  // Validates JWT
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateProductRequest>,
) -> Result<impl IntoResponse> {
    let collection_name = MongoDB::get_collection_name("MONGO_PRODUCTS_COLLECTION");
    let collection = state.collection(&collection_name);

    let product = ProductService::create_prouct(&collection, req).await?;

    let response = ApiResponse::success(serde_json::json!({
        "data": product
    }));

    Ok((StatusCode::CREATED, response))
}

// PUT /admin/products/:id (requires authentication)
pub async fn update_product(
    _auth: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateProductRequest>,
) -> Result<impl IntoResponse> {
    let collection_name = MongoDB::get_collection_name("MONGO_PRODUCTS_COLLECTION");
    let collection = state.collection(&collection_name);

    let product = ProductService::update_product(&collection, &id, req).await?;

    let response = ApiResponse::success(serde_json::json!({
        "data": product
    }));

    Ok(response)
}

// DELETE /admin/products/:id (requires authentication)
pub async fn delete_product(
    _auth: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let collection_name = MongoDB::get_collection_name("MONGO_PRODUCTS_COLLECTION");
    let collection = state.collection(&collection_name);

    ProductService::delete_product(&collection, &id).await?;

    let response = ApiResponse::with_message(serde_json::json!({}), "Product deleted successfully");

    Ok(response)
}
