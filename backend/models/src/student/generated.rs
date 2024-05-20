/* This file is generated and managed by dsync */

use crate::schema::*;
use diesel::QueryResult;
use diesel::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

type Connection = diesel_async::AsyncPgConnection;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=student, primary_key(id))]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub class: String,
    pub nis: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=student)]
pub struct CreateStudent {
    pub name: String,
    pub class: String,
    pub nis: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=student)]
pub struct UpdateStudent {
    pub name: Option<String>,
    pub class: Option<String>,
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
    pub async fn create(db: &mut Connection, item: &CreateStudent) -> QueryResult<Self> {
        use crate::schema::student::dsl::*;

        insert_into(student)
            .values(item)
            .get_result::<Self>(db)
            .await
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Self> {
        use crate::schema::student::dsl::*;

        student.filter(id.eq(param_id)).first::<Self>(db).await
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::student::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = student.count().get_result(db).await?;
        let items = student
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
        item: &UpdateStudent,
    ) -> QueryResult<Self> {
        use crate::schema::student::dsl::*;

        diesel::update(student.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
    }

    pub async fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::student::dsl::*;

        diesel::delete(student.filter(id.eq(param_id)))
            .execute(db)
            .await
    }
}
