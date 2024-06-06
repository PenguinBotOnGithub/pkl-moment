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

#[derive(Serialize, Clone, Debug)]
pub struct LogBrief {
    pub id: i32,
    pub operation_type: crate::types::Operation,
    pub table_affected: crate::types::TableRef,
    pub user: String,
    pub logged_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Clone, Debug)]
pub struct LogDetail {
    pub id: i32,
    pub operation_type: crate::types::Operation,
    pub table_affected: crate::types::TableRef,
    pub user: crate::user::UserPublic,
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

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<LogDetail>> {
        use crate::schema::log::dsl::*;
        use crate::schema::user;

        log.filter(id.eq(param_id))
            .inner_join(user::table)
            .select((
                id,
                operation_type,
                table_affected,
                snapshot,
                logged_at,
                user::dsl::id,
                user::dsl::username,
                user::dsl::role,
            ))
            .first::<(
                i32,
                crate::types::Operation,
                crate::types::TableRef,
                String,
                chrono::DateTime<chrono::Utc>,
                i32,
                String,
                crate::types::UserRole,
            )>(db)
            .await
            .optional()?
            .map_or(Ok(None), |v| {
                Ok(Some(LogDetail {
                    id: v.0,
                    operation_type: v.1,
                    table_affected: v.2,
                    user: crate::user::UserPublic {
                        id: v.5,
                        username: v.6,
                        role: v.7,
                    },
                    snapshot: v.3,
                    logged_at: v.4,
                }))
            })
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
        param_user_id: Option<i32>,
    ) -> QueryResult<PaginationResult<LogBrief>> {
        use crate::schema::log::dsl::*;
        use crate::schema::user;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = log.count().get_result(db).await?;
        let items = match param_user_id {
            Some(n) => log
                .filter(user_id.eq(n))
                .inner_join(user::table)
                .select((
                    id,
                    operation_type,
                    table_affected,
                    logged_at,
                    user::dsl::username,
                ))
                .limit(page_size)
                .offset(page * page_size)
                .load::<(
                    i32,
                    crate::types::Operation,
                    crate::types::TableRef,
                    chrono::DateTime<chrono::Utc>,
                    String,
                )>(db)
                .await?
                .into_iter()
                .map(|v| LogBrief {
                    id: v.0,
                    operation_type: v.1,
                    table_affected: v.2,
                    user: v.4,
                    logged_at: v.3,
                })
                .collect(),
            None => log
                .limit(page_size)
                .offset(page * page_size)
                .inner_join(user::table)
                .select((
                    id,
                    operation_type,
                    table_affected,
                    logged_at,
                    user::dsl::username,
                ))
                .load::<(
                    i32,
                    crate::types::Operation,
                    crate::types::TableRef,
                    chrono::DateTime<chrono::Utc>,
                    String,
                )>(db)
                .await?
                .into_iter()
                .map(|v| LogBrief {
                    id: v.0,
                    operation_type: v.1,
                    table_affected: v.2,
                    user: v.4,
                    logged_at: v.3,
                })
                .collect(),
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
}
