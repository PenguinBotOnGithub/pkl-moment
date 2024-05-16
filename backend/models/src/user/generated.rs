/* This file is generated and managed by dsync */

use crate::schema::*;
use diesel::QueryResult;
use diesel::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

type Connection = diesel_async::AsyncPgConnection;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=user, primary_key(id))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
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
    pub async fn create(db: &mut Connection, item: &CreateUser) -> QueryResult<Self> {
        use crate::schema::user::dsl::*;

        insert_into(user).values(item).get_result::<Self>(db).await
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
    ) -> QueryResult<Self> {
        use crate::schema::user::dsl::*;

        diesel::update(user.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
    }

    pub async fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::user::dsl::*;

        diesel::delete(user.filter(id.eq(param_id)))
            .execute(db)
            .await
    }
}
