use chrono::{DateTime, Utc};
use mongodb::bson::{bson, oid::ObjectId};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub items: Vec<OrderItem>,
    pub total_amount: f64,
    pub payment_method: String,  // "paystack", "opay", "offline"
    pub payment_reference: Option<String>,
    pub payment_status: String,  // "pending", "completed", "failed"
    pub order_status: String,    // "pending", "processing", "shipped", "completed", "cancelled"
    pub shipping_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: String,
    pub product_name: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub payment_method: String,
    pub shipping_address: Option<String>,
}
