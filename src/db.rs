use mongodb::{Collection, Database};

use crate::config::database::MongoDB;



#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Database
}

impl AppState {
    pub async fn init() -> Result<Self, mongodb::error::Error> {
        let mongodb = MongoDB::init().await?;
        Ok(AppState {
            db: mongodb.db
        })
    }

    pub fn collection<T: Send + Sync>(&self, collection_name: &str) -> Collection<T> {
         self.db.collection(collection_name)
    }
}