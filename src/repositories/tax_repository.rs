use sqlx::{Executor, Postgres};
use uuid::Uuid;
use crate::models::tax_model::{TaxDocumentModel, TaxDataModel};
use crate::utilities::error_bag::ErrorBag;

pub struct TaxRepository;

impl TaxRepository {
    pub async fn find_documents_by_user<'e, E: Executor<'e, Database = Postgres>>(
        db: E,
        user_id: &Uuid,
    ) -> Result<Vec<TaxDocumentModel>, ErrorBag> {
        Ok(sqlx::query_as!(
            TaxDocumentModel,
            "SELECT * FROM tax_documents WHERE user_id = $1 AND deleted_at IS NULL ORDER BY year DESC, created_at DESC",
            user_id
        )
        .fetch_all(db)
        .await?)
    }

    pub async fn create_document<'e, E: Executor<'e, Database = Postgres>>(
        db: E,
        user_id: &Uuid,
        year: i32,
        doc_type: &str,
        file_name: &str,
        file_path: &str,
    ) -> Result<TaxDocumentModel, ErrorBag> {
        Ok(sqlx::query_as!(
            TaxDocumentModel,
            "INSERT INTO tax_documents (user_id, year, document_type, file_name, file_path) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            user_id,
            year,
            doc_type,
            file_name,
            file_path
        )
        .fetch_one(db)
        .await?)
    }

    pub async fn find_data_by_user_and_year<'e, E: Executor<'e, Database = Postgres>>(
        db: E,
        user_id: &Uuid,
        year: i32,
    ) -> Result<Option<TaxDataModel>, ErrorBag> {
        Ok(sqlx::query_as!(
            TaxDataModel,
            "SELECT * FROM tax_data WHERE user_id = $1 AND year = $2 AND deleted_at IS NULL",
            user_id,
            year
        )
        .fetch_optional(db)
        .await?)
    }

    pub async fn find_all_data_by_user<'e, E: Executor<'e, Database = Postgres>>(
        db: E,
        user_id: &Uuid,
    ) -> Result<Vec<TaxDataModel>, ErrorBag> {
        Ok(sqlx::query_as!(
            TaxDataModel,
            "SELECT * FROM tax_data WHERE user_id = $1 AND deleted_at IS NULL ORDER BY year DESC",
            user_id
        )
        .fetch_all(db)
        .await?)
    }

    pub async fn upsert_tax_data<'e, E: Executor<'e, Database = Postgres>>(
        db: E,
        user_id: &Uuid,
        year: i32,
        data: &serde_json::Value,
    ) -> Result<TaxDataModel, ErrorBag> {
        Ok(sqlx::query_as!(
            TaxDataModel,
            r#"
            INSERT INTO tax_data (user_id, year, data)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, year)
            DO UPDATE SET data = EXCLUDED.data, updated_at = NOW()
            RETURNING *
            "#,
            user_id,
            year,
            data
        )
        .fetch_one(db)
        .await?)
    }

    pub async fn find_document_by_id<'e, E: Executor<'e, Database = Postgres>>(
        db: E,
        id: &Uuid,
    ) -> Result<TaxDocumentModel, ErrorBag> {
        Ok(sqlx::query_as!(
            TaxDocumentModel,
            "SELECT * FROM tax_documents WHERE id = $1 AND deleted_at IS NULL",
            id
        )
        .fetch_one(db)
        .await?)
    }
}
