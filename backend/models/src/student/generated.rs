/* This file is generated and managed by dsync */
use crate::class::Class;
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
#[diesel(table_name=student, primary_key(id), belongs_to(Class, foreign_key=class_id))]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub class_id: i32,
    pub nis: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct StudentJoined {
    pub id: i32,
    pub name: String,
    pub class: (i32, String),
    pub nis: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=student)]
pub struct CreateStudent {
    pub name: String,
    pub class_id: i32,
    pub nis: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=student)]
pub struct UpdateStudent {
    pub name: Option<String>,
    pub class_id: Option<i32>,
    pub nis: Option<String>,
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

impl Student {
    pub async fn create(
        db: &mut Connection,
        item: &CreateStudent,
        user_id: i32,
    ) -> QueryResult<Self> {
        use crate::schema::student::dsl::*;

        let res = diesel::insert_into(student)
            .values(item)
            .get_result::<Self>(db)
            .await;

        if let Ok(_) = res {
            Log::log(
                db,
                Operation::Create,
                TableRef::Student,
                user_id,
                None::<u8>,
            )
            .await;
        }

        res
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<StudentJoined>> {
        use crate::schema::class;
        use crate::schema::department;
        use crate::schema::student::dsl::*;

        let res = student
            .filter(id.eq(param_id))
            .inner_join(class::table.inner_join(department::table))
            .select((id, name, class::number, department::name, nis))
            .first::<(i32, String, i32, String, String)>(db)
            .await
            .optional()?;
        let Some(mut res) = res else { return Ok(None) };

        Ok(Some(StudentJoined {
            id: res.0,
            name: mem::take(&mut res.1),
            class: (res.2, mem::take(&mut res.3)),
            nis: mem::take(&mut res.4),
        }))
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<StudentJoined>> {
        use crate::schema::student::dsl::*;
        use crate::schema::class;
        use crate::schema::department;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = student.count().get_result(db).await?;
        let items = student
            .limit(page_size)
            .inner_join(class::table.inner_join(department::table))
            .select((id, name, class::number, department::name, nis))
            .offset(page * page_size)
            .load::<(i32, String, i32, String, String)>(db)
            .await?
            .iter_mut()
            .map(|(s_id, s_name, c_num, d_name, s_nis)| StudentJoined {
                id: *s_id,
                name: mem::take(s_name),
                class: (*c_num, mem::take(d_name)),
                nis: mem::take(s_nis),
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
        item: &UpdateStudent,
        user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::student::dsl::*;

        let previous = Student::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(student.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional();

        if let Ok(Some(_)) = res {
            Log::log(
                db,
                Operation::Update,
                TableRef::Student,
                user_id,
                Some(previous),
            )
            .await;
        }

        res
    }

    pub async fn delete(db: &mut Connection, param_id: i32, user_id: i32) -> QueryResult<usize> {
        use crate::schema::student::dsl::*;

        let previous = Student::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(student.filter(id.eq(param_id)))
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
                    TableRef::Student,
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
