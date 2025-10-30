use crate::models::product::{
    CreateProductRequest, Product, ProductFilter, ProductResponse, UpdateProductRequest,
};
use crate::utils::error::{AppError, Result};
use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::Collection;
use std::str::FromStr;
use futures_util::StreamExt;

pub struct ProductService;

impl ProductService{

    //create a new product 
    pub async fn create_prouct (
        collection: &Collection<Product>,
        req: CreateProductRequest
    ) -> Result<ProductResponse> { 

        let now = Utc::now();

        let product = Product {
            id: None,
            name: req.name,
            description: req.description,
            category: req.category,
            prodcut_type: req.product_type,
            price: req.price,
            stock_quantity: req.stock_quantity,
            cover_image: req.cover_image,
            aditional_images: req.additional_images,
            label: None,
            average_rating: 0.0,
            rating_count: 0,
            tags: req.tags,
            created_at: now,
            updated_at: now,
        };

        let result = collection.insert_one(product, ).await?;

        let inserted_id = result.inserted_id.as_object_id().ok_or_else(|| AppError::InternalError)?;


        //Fetch  the created product 
        let created_product = collection.find_one(doc! {"_id": inserted_id})
        .await?
        .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

    Ok(created_product.to_response())
    

}

pub async fn get_products(
    collection: &Collection<Product>,
    filter: Option<ProductFilter>,
    sort_by: Option<String>,
    page: i64,
    limit: i64,


) -> Result<Vec<ProductResponse>> {

    let mut query = Document::new();

    //Build filter query
    if let Some(f) = filter {
        if let Some(category) = f.category {
            query.insert("category", category);
        } 
        if let Some(product_type) = f.product_type {
                query.insert("product_type", product_type);
            }
            if let Some(tags) = f.tags {
                query.insert("tags", doc! { "$in": [tags] });
            }
            if f.min_price.is_some() || f.max_price.is_some() {
                let mut price_filter = Document::new();
                if let Some(min) = f.min_price {
                    price_filter.insert("$gte", min);
                }
                if let Some(max) = f.max_price {
                    price_filter.insert("$lte", max);
                }
                query.insert("price", price_filter);
            }
        }
           // Build sort options
        let sort_doc = match sort_by.as_deref() {
            Some("price_asc") => doc! { "price": 1 },
            Some("price_desc") => doc! { "price": -1 },
            Some("rating") => doc! { "average_rating": -1 },
            Some("newest") => doc! { "created_at": -1 },
            _ => doc! { "created_at": -1 },
        };
 let mut cursor = collection
            .find(query)
            .sort(sort_doc)
            .limit(limit)
            .skip(((page - 1) * limit) as u64)
            .await?;

        let mut products = Vec::new();
        while let Some(result) = cursor.next().await {
            let product = result?;
            products.push(product.to_response());
        }

        Ok(products)

    }

      // Search products by name or description
    pub async fn search_products(
        collection: &Collection<Product>,
        search_term: &str,
    ) -> Result<Vec<ProductResponse>> {
        let regex_pattern = format!(".*{}.*", search_term);
        
        let query = doc! {
            "$or": [
                { "name": { "$regex": regex_pattern.clone(), "$options": "i" } },
                { "description": { "$regex": regex_pattern, "$options": "i" } }
            ]
        };


        let mut cursor = collection.find(query).sort(doc! {
            "average_rating": -1
        })
        .limit(50)
        .await?;
        
        let mut products = Vec::new();
        while let Some(result) = cursor.next().await {
            let product = result?;
            products.push(product.to_response());
        }

        Ok(products)
    }

    // Get single product by ID
    pub async fn get_product_by_id(
        collection: &Collection<Product>,
        id: &str,
    ) -> Result<ProductResponse> {
        let object_id = ObjectId::from_str(id)
            .map_err(|_| AppError::ValidationError("Invalid product ID".to_string()))?;

        let product = collection
            .find_one(doc! { "_id": object_id }, )
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        Ok(product.to_response())
    }

    // Update product
    pub async fn update_product(
        collection: &Collection<Product>,
        id: &str,
        req: UpdateProductRequest,
    ) -> Result<ProductResponse> {
        let object_id = ObjectId::from_str(id)
            .map_err(|_| AppError::ValidationError("Invalid product ID".to_string()))?;

        let mut update_doc = Document::new();
        
        if let Some(name) = req.name {
            update_doc.insert("name", name);
        }
        if let Some(description) = req.description {
            update_doc.insert("description", description);
        }
        if let Some(price) = req.price {
            update_doc.insert("price", price);
        }
        if let Some(stock) = req.stock_quantity {
            update_doc.insert("stock_quantity", stock);
        }
        if let Some(tags) = req.tags {
            update_doc.insert("tags", tags);
        }
        
        update_doc.insert("updated_at", mongodb::bson::DateTime::from_millis(Utc::now().timestamp_millis()));

        collection
            .update_one(
                doc! { "_id": object_id },
                doc! { "$set": update_doc },
                
            )
            .await?;

        Self::get_product_by_id(collection, id).await
    }

    // Delete product
    pub async fn delete_product(
        collection: &Collection<Product>,
        id: &str,
    ) -> Result<()> {
        let object_id = ObjectId::from_str(id)
            .map_err(|_| AppError::ValidationError("Invalid product ID".to_string()))?;

        let result = collection
            .delete_one(doc! { "_id": object_id }, )
            .await?;

        if result.deleted_count == 0 {
            return Err(AppError::NotFound("Product not found".to_string()));
        }

        Ok(())
    }

}

