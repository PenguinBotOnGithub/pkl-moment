/* This file is generated and managed by dsync */
use crate::department::Department;
use crate::diesel::prelude::*;
use crate::log::Log;
use crate::schema::*;
use crate::types::{Operation, TableRef};
use diesel::QueryResult;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::mem;

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
#[diesel(table_name=class, primary_key(id), belongs_to(Department, foreign_key=department_id))]
pub struct Class {
    pub id: i32,
    pub number: i32,
    pub department_id: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClassJoined {
    pub id: i32,
    pub number: i32,
    pub department: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=class)]
pub struct CreateClass {
    pub number: i32,
    pub department_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=class)]
pub struct UpdateClass {
    pub number: Option<i32>,
    pub department_id: Option<i32>,
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

impl Class {
    pub async fn create(
        db: &mut Connection,
        item: &CreateClass,
        user_id: i32,
    ) -> QueryResult<Self> {
        use crate::schema::class::dsl::*;

        let res = diesel::insert_into(class)
            .values(item)
            .get_result::<Self>(db)
            .await;

        if let Ok(_) = res {
            Log::log(db, Operation::Create, TableRef::Class, user_id, None::<u8>).await;
        }

        res
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<Self>> {
        use crate::schema::class::dsl::*;

        class
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()
    }

    pub async fn read_joined(
        db: &mut Connection,
        param_id: i32,
    ) -> QueryResult<Option<ClassJoined>> {
        use crate::schema::class::dsl::*;
        use crate::schema::department;

        let res = class
            .filter(id.eq(param_id))
            .inner_join(department::table)
            .select((id, number, department::name))
            .first::<(i32, i32, String)>(db)
            .await
            .optional()?;

        let Some(mut res) = res else {
            return Ok(None);
        };

        Ok(Some(ClassJoined {
            id: res.0,
            number: res.1,
            department: mem::take(&mut res.2),
        }))
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<ClassJoined>> {
        use crate::schema::class::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = class.count().get_result(db).await?;
        let items = class
            .limit(page_size)
            .offset(page * page_size)
            .inner_join(department::table)
            .select((id, number, department::name))
            .load::<(i32, i32, String)>(db)
            .await?
            .iter_mut()
            .map(|v| ClassJoined {
                id: v.0,
                number: v.1,
                department: mem::take(&mut v.2),
            })
            .collect();

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
        item: &UpdateClass,
        user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::class::dsl::*;

        let previous = Class::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(class.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional();

        if let Ok(Some(_)) = res {
            Log::log(
                db,
                Operation::Update,
                TableRef::Class,
                user_id,
                Some(previous),
            )
            .await;
        }

        res
    }

    pub async fn delete(db: &mut Connection, param_id: i32, user_id: i32) -> QueryResult<usize> {
        use crate::schema::class::dsl::*;

        let previous = Class::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(class.filter(id.eq(param_id)))
            .execute(db)
            .await;

        match res {
            Ok(n) => {
                if n == 0 {
                    return res;
                }

                Log::log(
                    db,
                    Operation::Delete,
                    TableRef::Class,
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
