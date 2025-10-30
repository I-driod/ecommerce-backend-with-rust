use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub product_id: String,
    pub product_name: String,
    pub product_price: f64,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct AddToCartRequest {
    pub product_id: String,
    pub quantity: i32,
}
