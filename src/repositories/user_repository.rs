use crate::models::user_model::{UserModel};
use sqlx::{Acquire, Executor, Postgres};
use uuid::Uuid;
use crate::utilities::error_bag::ErrorBag;

/// UserModel repository encapsulates all SQL access for the `users` table.
///
/// Keeping queries in this layer makes controllers thin and focused on HTTP concerns.
pub struct UserRepository;

impl UserRepository {
    /// Find a user by their email where the account is not soft-deleted.
    ///
    /// Uses SQLx compile-time checked query macros ("sqlx micro") for safety.
    pub async fn find_by_email<'e, E: Executor<'e, Database = Postgres>>(db: E, email: &str) -> Result<UserModel, ErrorBag> {
        // Select all columns so we can hydrate the full domain model.
        Ok(
            sqlx::query_as!(
                UserModel,
                "SELECT * FROM users WHERE email = LOWER($1) AND deleted_at IS NULL LIMIT 1",
                email
            ).fetch_one(db).await?
        )
    }

    /// Find a user by their id where the account is not soft-deleted.
    ///
    /// Uses SQLx compile-time checked query macros ("sqlx micro") for safety.
    pub async fn find_by_id<'e, E: Executor<'e, Database = Postgres>>(db: E, id: &Uuid) -> Result<UserModel, ErrorBag> {
        // Select all columns so we can hydrate the full domain model.
        Ok(
            sqlx::query_as!(
                UserModel,
                "SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL LIMIT 1",
                id
            ).fetch_one(db).await?
        )
    }

    /// Updates an existing user record in the database.
    ///
    /// Takes a user model and updates all fields in the database. Returns the number of affected rows.
    /// Uses SQLx compile-time checked query for type safety.
    pub async fn update<'e, E: Executor<'e, Database = Postgres>>(db: E, user: &UserModel) -> Result<u64, ErrorBag> {
        // Use SQLx compile-time checked query and return affected rows for AI-friendly handling.
        Ok(
            sqlx::query!(
                r#"
                    UPDATE users
                    SET
                        first_name = $2,
                        last_name = $3,
                        email = LOWER($4),
                        phone = $5,
                        password = $6,
                        password_reset_token = $7,
                        verification_token = $8,
                        last_logged_in_at = $9,
                        current_logged_in_at = $10,
                        updated_at = NOW(),
                        deleted_at = $11
                    WHERE id = $1
                    "#,
                user.id,
                user.first_name,
                user.last_name,
                user.email,
                user.phone,
                user.password,
                user.password_reset_token,
                user.verification_token,
                user.last_logged_in_at,
                user.current_logged_in_at,
                user.deleted_at
            ).execute(db).await?.rows_affected()
        )
    }

    /// Checks if an email address already exists in the database.
    ///
    /// Returns true if email exists and account is not soft-deleted, false otherwise.
    pub async fn email_exist<'e, A: Acquire<'e, Database = Postgres>>(db: A, needle: &str) -> Result<bool, ErrorBag>
    {
        let mut conn = db.acquire().await?;
        Ok(sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = LOWER($1) AND deleted_at IS NULL)",
            needle
        ).fetch_one(&mut *conn).await?.exists.unwrap_or(false))
    }

    /// Creates a new user record in the database.
    ///
    /// Takes first name, last name, email and password hash and returns the created user.
    /// Sets current_logged_in_at to NOW() for new users.
    pub async fn create<'e, E: Executor<'e, Database = Postgres>>(
        db: E,
        f_name: &str,
        l_name: &str,
        email_address: &str,
        pass: &str,
    ) -> Result<UserModel, ErrorBag> {
        Ok(
            sqlx::query_as!(
                UserModel,
                "INSERT INTO users (first_name, last_name, email, password, current_logged_in_at) VALUES ($1, $2, LOWER($3), $4, NOW()) RETURNING *",
                f_name,
                l_name,
                email_address,
                pass
            ).fetch_one(db).await?
        )
    }

    /// Deletes a user record (soft delete) in the database.
    pub async fn delete<'e, E: Executor<'e, Database = Postgres>>(db: E, id: &Uuid) -> Result<u64, ErrorBag> {
        Ok(sqlx::query!(
            "UPDATE users SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            id
        ).execute(db).await?.rows_affected())
    }
}