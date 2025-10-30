use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub prodcut_type: String,
    pub price: f64,
    pub stock_quantity: i32,
    pub cover_image:Option<String>,
    pub aditional_images:Option<Vec<String>>,
    pub label: Option<String>,
    pub average_rating: f64,
    pub rating_count: i32,
    pub tags: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub product_type: String,
    pub price: f64,
    pub stock_quantity: i32,
    pub cover_image: Option<String>,
    pub additional_images: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock_quantity: Option<i32>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub product_type: String,
    pub price: f64,
    pub stock_quantity: i32,
    pub cover_image: Option<String>,
    pub additional_images: Option<Vec<String>>,
    pub label: Option<String>,
    pub average_rating: f64,
    pub rating_count: i32,
    pub tags: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
}

impl Product {
    // Convert Product to ProductResponse
    pub fn to_response(&self) -> ProductResponse {
        ProductResponse {
            id: self.id.unwrap().to_hex(),
            name: self.name.clone(),
            description: self.description.clone(),
            category: self.category.clone(),
            product_type: self.prodcut_type.clone(),
            price: self.price,
            stock_quantity: self.stock_quantity,
            cover_image: self.cover_image.clone(),
            additional_images: self.aditional_images.clone(),
            label: self.label.clone(),
            average_rating: self.average_rating,
            rating_count: self.rating_count,
            tags: self.tags.clone(),
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ProductFilter {
    pub category: Option<String>,
    pub product_type: Option<String>,
    pub tags: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
}

// Pagination parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 { 1 }
fn default_limit() -> i64 { 20 }

