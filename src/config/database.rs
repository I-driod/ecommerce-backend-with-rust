use std::env;

use mongodb::{options::ClientOptions, Client, Database};

pub struct MongoDB{
    pub db: Database
}

impl MongoDB {
    pub async fn init() -> Result<Self, mongodb::error::Error> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let database_name = env::var("MONGO_DATABASE").unwrap_or("ecommerce_db".to_string());

        let mut client_options = ClientOptions::parse(&database_url).await?;

        client_options.app_name = Some("EcommerceBackend".to_string());

        let client = Client::with_options(client_options)?;

        let db = client.database(&database_name);
        db.run_command(mongodb::bson::doc! { "ping": 1 }, ).await?;


        println!("✅ MongoDB connected successfully to: {}", database_name);



        Ok(MongoDB { db })
    
    }

      pub fn get_collection_name(key: &str) -> String {
        env::var(key).unwrap_or_else(|_| {
            match key {
                "MONGO_PRODUCTS_COLLECTION" => "products",
                "MONGO_USERS_COLLECTION" => "users",
                "MONGO_ORDERS_COLLECTION" => "orders",
                "MONGO_CART_COLLECTION" => "cart",
                "MONGO_REVIEWS_COLLECTION" => "reviews",
                _ => "default",
            }
            .to_string()
        })
    }
}