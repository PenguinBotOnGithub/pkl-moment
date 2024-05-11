/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use diesel_async::RunQueryDsl;
use crate::models::penarikan::Penarikan;
use crate::models::student::Student;

type Connection = diesel_async::AsyncPgConnection;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations, Selectable)]
#[diesel(table_name=penarikan_student, primary_key(id), belongs_to(Penarikan, foreign_key=penarikan_id) , belongs_to(Student, foreign_key=student_id))]
pub struct PenarikanStudent {
    pub id: i32,
    pub penarikan_id: i32,
    pub student_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=penarikan_student)]
pub struct CreatePenarikanStudent {
    pub id: i32,
    pub penarikan_id: i32,
    pub student_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=penarikan_student)]
pub struct UpdatePenarikanStudent {
    pub penarikan_id: Option<i32>,
    pub student_id: Option<i32>,
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

impl PenarikanStudent {

    pub async fn create(db: &mut Connection, item: &CreatePenarikanStudent) -> QueryResult<Self> {
        use crate::schema::penarikan_student::dsl::*;

        insert_into(penarikan_student).values(item).get_result::<Self>(db).await
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Self> {
        use crate::schema::penarikan_student::dsl::*;

        penarikan_student.filter(id.eq(param_id)).first::<Self>(db).await
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::penarikan_student::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = penarikan_student.count().get_result(db).await?;
        let items = penarikan_student.limit(page_size).offset(page * page_size).load::<Self>(db).await?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub async fn update(db: &mut Connection, param_id: i32, item: &UpdatePenarikanStudent) -> QueryResult<Self> {
        use crate::schema::penarikan_student::dsl::*;

        diesel::update(penarikan_student.filter(id.eq(param_id))).set(item).get_result(db).await
    }

    pub async fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::penarikan_student::dsl::*;

        diesel::delete(penarikan_student.filter(id.eq(param_id))).execute(db).await
    }

}