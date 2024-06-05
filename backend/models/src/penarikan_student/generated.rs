/* This file is generated and managed by dsync */

use diesel::QueryResult;
use diesel::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::penarikan::Penarikan;
use crate::schema::*;
use crate::student::Student;

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
#[diesel(table_name=penarikan_student, primary_key(id), belongs_to(Penarikan, foreign_key=penarikan_id) , belongs_to(Student, foreign_key=student_id))]
pub struct PenarikanStudent {
    pub id: i32,
    pub penarikan_id: i32,
    pub student_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=penarikan_student)]
pub struct CreatePenarikanStudent {
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

        insert_into(penarikan_student)
            .values(item)
            .get_result::<Self>(db)
            .await
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<Self>> {
        use crate::schema::penarikan_student::dsl::*;

        penarikan_student
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()
    }

    pub async fn filter_by_letter_and_return_letter_id(
        db: &mut Connection,
        param_letter_id: i32,
    ) -> QueryResult<Option<(i32, Vec<Student>)>> {
        use crate::schema::penarikan;
        use crate::schema::penarikan_student::dsl::*;
        use crate::schema::student;

        let letter = penarikan::dsl::penarikan
            .filter(penarikan::dsl::id.eq(param_letter_id))
            .select(penarikan::user_id)
            .first::<i32>(db)
            .await
            .optional()?;

        match letter {
            Some(n) => {
                let students = penarikan_student
                    .filter(penarikan_id.eq(param_letter_id))
                    .inner_join(student::table)
                    .inner_join(penarikan::table)
                    .select(student::all_columns)
                    .load::<Student>(db)
                    .await?;
                Ok(Some((n, students)))
            }
            None => Ok(None),
        }
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::penarikan_student::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = penarikan_student.count().get_result(db).await?;
        let items = penarikan_student
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
        item: &UpdatePenarikanStudent,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::penarikan_student::dsl::*;

        diesel::update(penarikan_student.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional()
    }

    pub async fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::penarikan_student::dsl::*;

        diesel::delete(penarikan_student.filter(id.eq(param_id)))
            .execute(db)
            .await
    }
}
