use crate::models::user::{Claims, LoginRequest, RegisterRequest, User, UserResponse};
use crate::utils::error::{AppError, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::doc;
use mongodb::Collection;

pub struct AuthService;

impl AuthService {
    // Register new user
    pub async fn register(
        collection: &Collection<User>,
        req: RegisterRequest,
    ) -> Result<User> {
        // Validate password length
        if req.password.len() < 8 {
            return Err(AppError::ValidationError(
                "Password must be at least 8 characters".to_string(),
            ));
        }

        // Check if user already exists
        let existing_user = collection
            .find_one(
                doc! {
                    "$or": [
                        { "email": &req.email },
                        { "phone": req.phone.as_ref().unwrap_or(&String::new()) }
                    ]
                },
               
            )
            .await?;

        if existing_user.is_some() {
            return Err(AppError::ValidationError(
                "Email or phone already registered".to_string(),
            ));
        }

        // Hash password
        let password_hash = hash(req.password.as_bytes(), DEFAULT_COST)
            .map_err(|_| AppError::InternalError)?;

        let user = User {
            id: None,
            email: req.email,
            phone: req.phone,
            password_hash,
            full_name: req.full_name,
            role: "customer".to_string(),
            created_at: Utc::now(),
        };

        let result = collection.insert_one(&user, ).await?;
        
        let user_id = result.inserted_id.as_object_id()
            .ok_or_else(|| AppError::InternalError)?;

        let created_user = collection
            .find_one(doc! { "_id": user_id }, )
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(created_user)
    }

    // Login user
    pub async fn login(
        collection: &Collection<User>,
        req: LoginRequest,
    ) -> Result<User> {
        // Find user by email or phone
        let user = collection
            .find_one(
                doc! {
                    "$or": [
                        { "email": &req.email_or_phone },
                        { "phone": &req.email_or_phone }
                    ]
                },
                
            )
            .await?
            .ok_or_else(|| AppError::AuthError("Invalid credentials".to_string()))?;

        // Verify password
        let is_valid = verify(req.password.as_bytes(), &user.password_hash)
            .map_err(|_| AppError::InternalError)?;

        if !is_valid {
            return Err(AppError::AuthError("Invalid credentials".to_string()));
        }

        Ok(user)
    }

    // Generate JWT token
    pub fn generate_jwt(user: &User, secret: &str) -> Result<String> {
        let now = Utc::now();
        let exp = (now + chrono::Duration::hours(24)).timestamp() as usize;
        let iat = now.timestamp() as usize;

        let claims = Claims {
            sub: user.id.unwrap().to_hex(),
            email: user.email.clone(),
            role: user.role.clone(),
            exp,
            iat,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|_| AppError::InternalError)
    }

    // Verify JWT token
    pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::AuthError("Invalid token".to_string()))
    }

    // Convert User to UserResponse
    pub fn user_to_response(user: &User) -> UserResponse {
        UserResponse {
            id: user.id.unwrap().to_hex(),
            email: user.email.clone(),
            full_name: user.full_name.clone(),
            role: user.role.clone(),
        }
    }
}
