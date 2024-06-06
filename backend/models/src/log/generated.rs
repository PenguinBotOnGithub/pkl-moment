/* This file is generated and managed by dsync */

use crate::schema::*;
use crate::user::User;
use diesel::QueryResult;
use diesel::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

type Connection = diesel_async::AsyncPgConnection;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Insertable,
    AsChangeset,
    Identifiable,
    Associations,
    Selectable,
)]
#[diesel(table_name=log, primary_key(id), belongs_to(User, foreign_key=user_id))]
pub struct Log {
    pub id: i32,
    pub operation_type: crate::types::Operation,
    pub table_affected: crate::types::TableRef,
    pub user_id: i32,
    pub snapshot: String,
    pub logged_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=log)]
pub struct CreateLog {
    pub operation_type: crate::types::Operation,
    pub table_affected: crate::types::TableRef,
    pub user_id: i32,
    pub snapshot: String,
    pub logged_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=log)]
pub struct UpdateLog {
    pub operation_type: Option<crate::types::Operation>,
    pub table_affected: Option<crate::types::TableRef>,
    pub user_id: Option<i32>,
    pub snapshot: Option<String>,
    pub logged_at: Option<chrono::DateTime<chrono::Utc>>,
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

impl Log {
    pub async fn create(db: &mut Connection, item: &CreateLog) -> QueryResult<Self> {
        use crate::schema::log::dsl::*;

        insert_into(log).values(item).get_result::<Self>(db).await
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<Self>> {
        use crate::schema::log::dsl::*;

        log.filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
        param_user_id: Option<i32>,
    ) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::log::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = log.count().get_result(db).await?;
        let items = match param_user_id {
            Some(n) => {
                log.filter(user_id.eq(n))
                    .limit(page_size)
                    .offset(page * page_size)
                    .load::<Self>(db)
                    .await?
            }
            None => {
                log.limit(page_size)
                    .offset(page * page_size)
                    .load::<Self>(db)
                    .await?
            }
        };

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0),
        })
    }

    pub async fn update(db: &mut Connection, param_id: i32, item: &UpdateLog) -> QueryResult<Self> {
        use crate::schema::log::dsl::*;

        diesel::update(log.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
    }

    pub async fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::log::dsl::*;

        diesel::delete(log.filter(id.eq(param_id)))
            .execute(db)
            .await
    }
}
