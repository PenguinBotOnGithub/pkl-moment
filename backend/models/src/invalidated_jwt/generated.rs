/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use diesel_async::RunQueryDsl;


type Connection = diesel_async::AsyncPgConnection;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=invalidated_jwt, primary_key(id))]
pub struct InvalidatedJwt {
    pub id: i32,
    pub jwt: String,
    pub invalidated_timestamp: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=invalidated_jwt)]
pub struct CreateInvalidatedJwt {
    pub id: i32,
    pub jwt: String,
    pub invalidated_timestamp: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=invalidated_jwt)]
pub struct UpdateInvalidatedJwt {
    pub jwt: Option<String>,
    pub invalidated_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
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

impl InvalidatedJwt {

    pub async fn create(db: &mut Connection, item: &CreateInvalidatedJwt) -> QueryResult<Self> {
        use crate::schema::invalidated_jwt::dsl::*;

        insert_into(invalidated_jwt).values(item).get_result::<Self>(db).await
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Self> {
        use crate::schema::invalidated_jwt::dsl::*;

        invalidated_jwt.filter(id.eq(param_id)).first::<Self>(db).await
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::invalidated_jwt::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = invalidated_jwt.count().get_result(db).await?;
        let items = invalidated_jwt.limit(page_size).offset(page * page_size).load::<Self>(db).await?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub async fn update(db: &mut Connection, param_id: i32, item: &UpdateInvalidatedJwt) -> QueryResult<Self> {
        use crate::schema::invalidated_jwt::dsl::*;

        diesel::update(invalidated_jwt.filter(id.eq(param_id))).set(item).get_result(db).await
    }

    pub async fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::invalidated_jwt::dsl::*;

        diesel::delete(invalidated_jwt.filter(id.eq(param_id))).execute(db).await
    }

}