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
    Letters,
    LettersStudent,
    Class,
    Department,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Copy, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Operation"]
pub enum Operation {
    Create,
    Update,
    Delete,
    Verify,
    Register,
    Upload,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Copy)]
#[ExistingTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRole {
    Secretary,
    Coordinator,
}

impl Serialize for UserRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            UserRole::Secretary => serializer.serialize_unit_variant("UserRole", 0, "secretary"),
            UserRole::Coordinator => {
                serializer.serialize_unit_variant("UserRole", 0, "coordinator")
            }
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
                formatter.write_str("expected values: 'secretary' or 'coordinator'")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    "secretary" => Ok(UserRole::Secretary),
                    "coordinator" => Ok(UserRole::Coordinator),
                    _ => Err(E::custom(format!("unknown variant: {v}"))),
                }
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    "secretary" => Ok(UserRole::Secretary),
                    "coordinator" => Ok(UserRole::Coordinator),
                    _ => Err(E::custom(format!("unknown variant: {v}"))),
                }
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match &v[..] {
                    "secretary" => Ok(UserRole::Secretary),
                    "coordinator" => Ok(UserRole::Coordinator),
                    _ => Err(E::custom(format!("unknown variant: {v}"))),
                }
            }
        }

        deserializer.deserialize_string(UserRoleVisitor)
    }
}
