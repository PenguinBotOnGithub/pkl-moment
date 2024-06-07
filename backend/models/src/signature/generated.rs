/* This file is generated and managed by dsync */

use crate::log::{CreateLog, Log};
use crate::schema::*;
use diesel::QueryResult;
use diesel::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::types::{Operation, TableRef};

type Connection = diesel_async::AsyncPgConnection;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=signature, primary_key(id))]
pub struct Signature {
    pub id: i32,
    pub name: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=signature)]
pub struct CreateSignature {
    pub name: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=signature)]
pub struct UpdateSignature {
    pub name: Option<String>,
    pub title: Option<String>,
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

impl Signature {
    pub async fn create(
        db: &mut Connection,
        item: &CreateSignature,
        user_id: i32,
    ) -> QueryResult<Self> {
        use crate::schema::signature::dsl::*;

        let res = insert_into(signature)
            .values(item)
            .get_result::<Self>(db)
            .await;

        let Ok(_) = res.as_ref() else {
            return res;
        };

        if let Err(e) = Log::create(
            db,
            &CreateLog {
                operation_type: Operation::Create,
                table_affected: TableRef::Signature,
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
        use crate::schema::signature::dsl::*;

        signature
            .filter(id.eq(param_id))
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
        use crate::schema::signature::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = signature.count().get_result(db).await?;
        let items = signature
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
        item: &UpdateSignature,
        user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::signature::dsl::*;

        let previous = signature
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(signature.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional();

        let Ok(_) = res.as_ref() else {
            return res;
        };

        if let Err(e) = Log::create(
            db,
            &CreateLog {
                operation_type: Operation::Update,
                table_affected: TableRef::Signature,
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
        use crate::schema::signature::dsl::*;

        let previous = signature
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(signature.filter(id.eq(param_id)))
            .execute(db)
            .await;

        let Ok(_) = res.as_ref() else {
            return res;
        };

        if let Err(e) = Log::create(
            db,
            &CreateLog {
                operation_type: Operation::Delete,
                table_affected: TableRef::Signature,
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
