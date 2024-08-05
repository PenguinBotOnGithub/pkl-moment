/* This file is generated and managed by dsync */

use crate::diesel::prelude::*;
use crate::schema::*;
use crate::types::{Operation, TableRef};
use crate::user::User;
use diesel::QueryResult;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::error;

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
    pub operation_type: Operation,
    pub table_affected: TableRef,
    pub user_id: i32,
    pub snapshot: Option<String>,
    pub logged_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Clone, Debug)]
pub struct LogBrief {
    pub id: i32,
    pub operation_type: Operation,
    pub table_affected: TableRef,
    pub user: String,
    pub logged_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Clone, Debug)]
pub struct LogDetail {
    pub id: i32,
    pub operation_type: Operation,
    pub table_affected: TableRef,
    pub user: crate::user::UserPublic,
    pub snapshot: Option<String>,
    pub logged_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=log)]
pub struct CreateLog {
    pub operation_type: Operation,
    pub table_affected: TableRef,
    pub user_id: i32,
    pub snapshot: Option<String>,
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
    pub async fn log(
        db: &mut Connection,
        op_type: Operation,
        t_ref: TableRef,
        u_id: i32,
        ss: Option<impl serde::Serialize>,
    ) {
        if let Err(e) = Log::create(
            db,
            &CreateLog {
                operation_type: op_type,
                table_affected: t_ref,
                user_id: u_id,
                snapshot: match ss {
                    None => None,
                    Some(v) => match serde_json::to_string(&v) {
                        Ok(v) => Some(v),
                        Err(e) => {
                            error!("error serializing snapshot to json: {}", e.to_string());
                            Some(format!(
                                "error serializing snapshot to json: {}",
                                e.to_string()
                            ))
                        }
                    },
                },
            },
        )
        .await
        {
            error!("error logging action: {}", e.to_string());
        }
    }

    pub async fn create(db: &mut Connection, item: &CreateLog) -> QueryResult<Self> {
        use crate::schema::log::dsl::*;

        diesel::insert_into(log)
            .values(item)
            .get_result::<Self>(db)
            .await
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
                Operation,
                TableRef,
                Option<String>,
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
                    Operation,
                    TableRef,
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
                    Operation,
                    TableRef,
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
