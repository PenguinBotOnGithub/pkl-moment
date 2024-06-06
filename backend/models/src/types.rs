use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Copy, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::TableRef"]
pub enum TableRef {
    User,
    Wave,
    Company,
    Student,
    Signature,
    Permohonan,
    PermohonanStudent,
    Pengantaran,
    PengantaranStudent,
    Penarikan,
    PenarikanStudent,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Copy, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Operation"]
pub enum Operation {
    Create,
    Update,
    Delete,
    Verify,
    Unverify,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Copy)]
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

impl<'de> Deserialize<'de> for UserRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserRoleVisitor;

        impl<'de> Visitor<'de> for UserRoleVisitor {
            type Value = UserRole;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("expected values: 'admin' or 'string'")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    "admin" => Ok(UserRole::Admin),
                    "advisor" => Ok(UserRole::Advisor),
                    _ => Err(E::custom(format!("unknown variant: {v}"))),
                }
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    "admin" => Ok(UserRole::Admin),
                    "advisor" => Ok(UserRole::Advisor),
                    _ => Err(E::custom(format!("unknown variant: {v}"))),
                }
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match &v[..] {
                    "admin" => Ok(UserRole::Admin),
                    "advisor" => Ok(UserRole::Advisor),
                    _ => Err(E::custom(format!("unknown variant: {v}"))),
                }
            }
        }

        deserializer.deserialize_string(UserRoleVisitor)
    }
}
