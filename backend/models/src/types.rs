use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRole {
    Admin,
    Advisor,
}

impl Serialize for UserRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            UserRole::Admin => serializer.serialize_unit_variant("UserRole", 0, "admin"),
            UserRole::Advisor => serializer.serialize_unit_variant("UserRole", 0, "advisor"),
        }
    }
}
