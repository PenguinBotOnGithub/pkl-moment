/* This file is generated and managed by dsync */

use diesel::QueryResult;
use diesel::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::pengantaran::Pengantaran;
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
#[diesel(table_name=pengantaran_student, primary_key(id), belongs_to(Pengantaran, foreign_key=pengantaran_id) , belongs_to(Student, foreign_key=student_id))]
pub struct PengantaranStudent {
    pub id: i32,
    pub pengantaran_id: i32,
    pub student_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=pengantaran_student)]
pub struct CreatePengantaranStudent {
    pub pengantaran_id: i32,
    pub student_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=pengantaran_student)]
pub struct UpdatePengantaranStudent {
    pub pengantaran_id: Option<i32>,
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

impl PengantaranStudent {
    pub async fn create(db: &mut Connection, item: &CreatePengantaranStudent) -> QueryResult<Self> {
        use crate::schema::pengantaran_student::dsl::*;

        insert_into(pengantaran_student)
            .values(item)
            .get_result::<Self>(db)
            .await
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<Self>> {
        use crate::schema::pengantaran_student::dsl::*;

        pengantaran_student
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()
    }

    pub async fn filter_by_letter_and_return_letter_id(
        db: &mut Connection,
        param_letter_id: i32,
    ) -> QueryResult<Option<(i32, Vec<Student>)>> {
        use crate::schema::pengantaran;
        use crate::schema::pengantaran_student::dsl::*;
        use crate::schema::student;

        let letter = pengantaran::dsl::pengantaran
            .filter(pengantaran::dsl::id.eq(param_letter_id))
            .select(pengantaran::user_id)
            .first::<i32>(db)
            .await
            .optional()?;

        match letter {
            Some(n) => {
                let students = pengantaran_student
                    .filter(pengantaran_id.eq(param_letter_id))
                    .inner_join(student::table)
                    .inner_join(pengantaran::table)
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
        use crate::schema::pengantaran_student::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = pengantaran_student.count().get_result(db).await?;
        let items = pengantaran_student
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
        item: &UpdatePengantaranStudent,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::pengantaran_student::dsl::*;

        diesel::update(pengantaran_student.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional()
    }

    pub async fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::pengantaran_student::dsl::*;

        diesel::delete(pengantaran_student.filter(id.eq(param_id)))
            .execute(db)
            .await
    }

    pub async fn delete_by_student_and_letter_id(
        db: &mut Connection,
        param_student_id: i32,
        param_letter_id: i32,
    ) -> QueryResult<usize> {
        use crate::schema::pengantaran_student::dsl::*;

        diesel::delete(pengantaran_student.filter(student_id.eq(param_student_id)))
            .filter(pengantaran_id.eq(param_letter_id))
            .execute(db)
            .await
    }
}
