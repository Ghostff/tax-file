#[macro_export]
macro_rules! impl_model {
    ($type:ty, $table:expr, $id_field:ident) => {
        impl Model for $type {
            fn table(&self) -> &'static str {
                $table
            }

            fn id(&self) -> &uuid::Uuid {
                &self.$id_field
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}