/* This file is generated and managed by dsync */

use crate::log::{CreateLog, Log};
use crate::schema::*;
use crate::types::{Operation, TableRef};
use diesel::QueryResult;
use diesel::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::error;

type Connection = diesel_async::AsyncPgConnection;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=wave, primary_key(id))]
pub struct Wave {
    pub id: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=wave)]
pub struct CreateWave {
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=wave)]
pub struct UpdateWave {
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
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

impl Wave {
    pub async fn create(db: &mut Connection, item: &CreateWave, user_id: i32) -> QueryResult<Self> {
        use crate::schema::wave::dsl::*;

        let res = insert_into(wave).values(item).get_result::<Self>(db).await;

        let Ok(_) = res.as_ref() else {
            return res;
        };

        if let Err(e) = Log::create(
            db,
            &CreateLog {
                operation_type: Operation::Create,
                table_affected: TableRef::Wave,
                user_id,
                snapshot: None,
            },
        )
        .await
        {
            error!("error logging action: {}", e.to_string());
        }

        res
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<Self>> {
        use crate::schema::wave::dsl::*;

        wave.filter(id.eq(param_id))
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
        use crate::schema::wave::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = wave.count().get_result(db).await?;
        let items = wave
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
        item: &UpdateWave,
        user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::wave::dsl::*;

        let previous = wave
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(wave.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional();

        let Ok(_) = res else {
            return res;
        };

        if let Err(e) = Log::create(
            db,
            &CreateLog {
                operation_type: Operation::Update,
                table_affected: TableRef::Wave,
                user_id,
                snapshot: match serde_json::to_string(&previous) {
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
        )
        .await
        {
            error!("error logging action: {}", e.to_string());
        }

        res
    }

    pub async fn delete(db: &mut Connection, param_id: i32, user_id: i32) -> QueryResult<usize> {
        use crate::schema::wave::dsl::*;

        let previous = wave
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(wave.filter(id.eq(param_id)))
            .execute(db)
            .await;

        let Ok(_) = res.as_ref() else {
            return res;
        };

        if let Err(e) = Log::create(
            db,
            &CreateLog {
                operation_type: Operation::Delete,
                table_affected: TableRef::Wave,
                user_id,
                snapshot: match serde_json::to_string(&previous) {
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
        )
        .await
        {
            error!("error logging action: {}", e.to_string());
        }

        res
    }
}
