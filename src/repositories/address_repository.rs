use sqlx::{Executor, Postgres};
use rust_decimal::Decimal;
use crate::models::address_model::AddressModel;
use crate::utilities::error_bag::ErrorBag;

pub struct AddressRepository;

impl AddressRepository {
    /// Finds an existing address or creates a new one
    pub async fn find_or_create<'e, E>(
        db: E,
        address_raw: &str,
        street_address: Option<&str>,
        locality: Option<&str>,
        state_code: Option<&str>,
        zip: Option<&str>,
        country_code: &str,
        latitude: Option<Decimal>,
        longitude: Option<Decimal>,
    ) -> Result<AddressModel, ErrorBag>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let address = sqlx::query_as!(
            AddressModel,
            r#"
            WITH country AS ( SELECT id FROM countries WHERE iso3_code = $5 OR iso2_code = $5 LIMIT 1),
            state AS (SELECT s.id FROM states s JOIN country c ON s.country_id = c.id WHERE s.code = $6 LIMIT 1)
            INSERT INTO addresses (
                address_raw,
                street_address,
                latitude,
                longitude,
                country_id,
                state_id,
                locality,
                zip
            )
            SELECT
                $1,
                $2,
                $3,
                $4,
                c.id,
                st.id,
                $7,
                $8
            FROM country c
            LEFT JOIN state st ON TRUE
            ON CONFLICT (address_raw, locality, zip, country_id)
            WHERE deleted_at IS NULL
            DO UPDATE SET address_raw = EXCLUDED.address_raw
            RETURNING *
            "#,
            address_raw,
            street_address,
            latitude,
            longitude,
            country_code,
            state_code,
            locality,
            zip
        ).fetch_one(db).await?;

        Ok(address)
    }


}
