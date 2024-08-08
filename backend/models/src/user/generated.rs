/* This file is generated and managed by dsync */

use crate::diesel::prelude::*;
use crate::log::Log;
use crate::schema::*;
use crate::types::{Operation, TableRef, UserRole};
use diesel::QueryResult;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::mem;

type Connection = diesel_async::AsyncPgConnection;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=user, primary_key(id))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: crate::types::UserRole,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPublic {
    pub id: i32,
    pub username: String,
    pub role: crate::types::UserRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=user)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub role: crate::types::UserRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=user)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub role: Option<crate::types::UserRole>,
}

#[derive(Debug, Serialize)]
pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    /// 0-based index
    pub page: i64,
    pub page_size: i64,
    pub num_pages: i64,
}

impl User {
    pub fn public(&mut self) -> UserPublic {
        UserPublic {
            id: self.id,
            username: mem::take(&mut self.username),
            role: self.role,
        }
    }

    pub async fn create(db: &mut Connection, item: &CreateUser, user_id: i32) -> QueryResult<Self> {
        use crate::schema::user::dsl::*;

        let res = diesel::insert_into(user)
            .values(item)
            .get_result::<Self>(db)
            .await;

        if let Ok(_) = res {
            Log::log(db, Operation::Create, TableRef::User, user_id, None::<u8>).await;
        }

        res
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<Self>> {
        use crate::schema::user::dsl::*;

        user.filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()
    }

    pub async fn find_by_username(
        db: &mut Connection,
        param_username: &str,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::user::dsl::*;

        user.filter(username.eq(param_username))
            .first::<Self>(db)
            .await
            .optional()
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::user::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = user.count().get_result(db).await?;
        let items = user
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(db)
            .await?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0),
        })
    }

    pub async fn update(
        db: &mut Connection,
        param_id: i32,
        item: &UpdateUser,
        user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::user::dsl::*;

        let previous = user
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(user.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional();

        if let Ok(Some(_)) = res {
            Log::log(
                db,
                Operation::Update,
                TableRef::User,
                user_id,
                Some(previous),
            )
            .await;
        }

        res
    }

    pub async fn delete(db: &mut Connection, param_id: i32, user_id: i32) -> QueryResult<usize> {
        use crate::schema::user::dsl::*;

        let previous = user
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(user.filter(id.eq(param_id)))
            .execute(db)
            .await;

        match res {
            Ok(n) => {
                if n <= 0 {
                    return res;
                }

                Log::log(
                    db,
                    Operation::Delete,
                    TableRef::User,
                    user_id,
                    Some(previous),
                )
                .await;

                res
            }
            Err(_) => res,
        }
    }

    pub async fn find_by_username_role<'a>(
        db: &mut Connection,
        param_username: &str,
        param_role: &String,
    ) -> QueryResult<Vec<UserPublic>> {
        use crate::schema::user::dsl::*;

        let q = match &param_role[..] {
            "secretary" => user.filter(role.eq(UserRole::Secretary)).into_boxed(),
            "coordinator" => user.filter(role.eq(UserRole::Coordinator)).into_boxed(),
            "advisor" => user
                .filter(
                    role.eq(UserRole::AdvisorDudi)
                        .or(role.eq(UserRole::AdvisorSchool)),
                )
                .into_boxed(),
            "student" => user.filter(role.eq(UserRole::Student)).into_boxed(),
            _ => user.into_boxed(),
        };

        let res: Vec<UserPublic> = q
            .filter(username.ilike(format!("%{param_username}%")))
            .load::<Self>(db)
            .await?
            .into_iter()
            .map(|mut v| v.public())
            .collect();

        Ok(res)
    }
}
