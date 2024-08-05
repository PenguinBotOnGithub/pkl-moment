/* This file is generated and managed by dsync */

use crate::schema::*;
use crate::types::{Operation, TableRef};
use crate::{diesel::prelude::*, log::Log};
use diesel::QueryResult;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

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

        let res = diesel::insert_into(signature)
            .values(item)
            .get_result::<Self>(db)
            .await;

        if let Ok(_) = res {
            Log::log(
                db,
                Operation::Create,
                TableRef::Signature,
                user_id,
                None::<u8>,
            )
            .await;
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

        let previous = Signature::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(signature.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional();

        if let Ok(Some(_)) = res {
            Log::log(
                db,
                Operation::Update,
                TableRef::Signature,
                user_id,
                Some(previous),
            )
            .await;
        }

        res
    }

    pub async fn delete(db: &mut Connection, param_id: i32, user_id: i32) -> QueryResult<usize> {
        use crate::schema::signature::dsl::*;

        let previous = Signature::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(signature.filter(id.eq(param_id)))
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
                    TableRef::Signature,
                    user_id,
                    Some(previous),
                )
                .await;

                res
            }
            Err(_) => res,
        }
    }
}
