use crate::utilities::error_bag::ErrorBag;

pub fn fill_from_json<T>(data: serde_json::Value, ) -> Result<T, ErrorBag>
where
    T: serde::de::DeserializeOwned,
{
    let value: T = serde_json::from_value(data)
        .map_err(|e| ErrorBag::Json(e.to_string()))?;

    Ok(value)
}