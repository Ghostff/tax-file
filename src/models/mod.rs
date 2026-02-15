use std::any::Any;
use erased_serde::serialize_trait_object;

pub mod user_model;
pub mod address_model;
mod macros;

pub trait Model: erased_serde::Serialize + Any + Sync + Send {
    fn table(&self) -> &'static str;
    fn id(&self) -> &uuid::Uuid;
    fn as_any(&self) -> &dyn Any;
}

serialize_trait_object!(Model);
