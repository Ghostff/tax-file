use sqlx::{Acquire, Postgres};
use crate::models::user_model::{CreateUserSchema, UserModel};
use crate::repositories::user_repository::UserRepository;
use crate::services::crypto_service::CryptoService;
use crate::utilities::error_bag::ErrorBag;
use crate::utilities::str::FilterEmptyString;

pub struct UserService;


impl UserService {
    pub async fn create<'e, A: Acquire<'e, Database = Postgres>>(db: A, body: &CreateUserSchema) -> Result<UserModel, ErrorBag>
    {
        let password = body.password.as_deref().empty_as_none();
        let email = body.email.trim().to_lowercase();

        let password = match password {
            Some(p) => p,
            None => return Err(ErrorBag::Validation { field: "password".into(), message: "password is required".into() }),
        };

        // Step 1: Early email existence check
        let mut conn = db.acquire().await?;
        if UserRepository::email_exist(&mut *conn, &email).await? {
            return Err(ErrorBag::EmailInUse);
        }

        // Step 2: Hash password (Argon2)
        let crypto = CryptoService::new();
        let password = crypto.hash_password(password)
            .map_err(|e| ErrorBag::InternalServerError(format!("UserService::create.hash_password failed: {:?}", e)))?;

        // Step 3: Insert user
        UserRepository::create(
            &mut *conn,
            &body.first_name,
            &body.last_name,
            &email,
            &password,
        ).await
    }
}