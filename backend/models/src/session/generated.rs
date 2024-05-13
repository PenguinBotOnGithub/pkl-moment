/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use diesel_async::RunQueryDsl;
use crate::models::user::User;

type Connection = diesel_async::AsyncPgConnection;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations, Selectable)]
#[diesel(table_name=session, primary_key(id), belongs_to(User, foreign_key=user_id))]
pub struct Session {
    pub id: uuid::Uuid,
    pub user_id: i32,
    pub invalidated: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=session)]
pub struct CreateSession {
    pub id: uuid::Uuid,
    pub user_id: i32,
    pub invalidated: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=session)]
pub struct UpdateSession {
    pub user_id: Option<i32>,
    pub invalidated: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
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

impl Session {

    pub async fn create(db: &mut Connection, item: &CreateSession) -> QueryResult<Self> {
        use crate::schema::session::dsl::*;

        insert_into(session).values(item).get_result::<Self>(db).await
    }

    pub async fn read(db: &mut Connection, param_id: uuid::Uuid) -> QueryResult<Self> {
        use crate::schema::session::dsl::*;

        session.filter(id.eq(param_id)).first::<Self>(db).await
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::session::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = session.count().get_result(db).await?;
        let items = session.limit(page_size).offset(page * page_size).load::<Self>(db).await?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub async fn update(db: &mut Connection, param_id: uuid::Uuid, item: &UpdateSession) -> QueryResult<Self> {
        use crate::schema::session::dsl::*;

        diesel::update(session.filter(id.eq(param_id))).set(item).get_result(db).await
    }

    pub async fn delete(db: &mut Connection, param_id: uuid::Uuid) -> QueryResult<usize> {
        use crate::schema::session::dsl::*;

        diesel::delete(session.filter(id.eq(param_id))).execute(db).await
    }

}