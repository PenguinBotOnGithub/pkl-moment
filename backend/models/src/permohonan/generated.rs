/* This file is generated and managed by dsync */

use crate::company::Company;
use crate::schema::*;
use crate::user::User;
use crate::wave::Wave;
use diesel::QueryResult;
use diesel::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

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
#[diesel(table_name=permohonan, primary_key(id), belongs_to(Company, foreign_key=company_id) , belongs_to(User, foreign_key=user_id) , belongs_to(Wave, foreign_key=wave_id))]
pub struct Permohonan {
    pub id: i32,
    pub user_id: i32,
    pub company_id: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub verified: bool,
    pub verified_date: Option<chrono::NaiveDate>,
    pub wave_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=permohonan)]
pub struct CreatePermohonan {
    pub user_id: Option<i32>,
    pub company_id: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub verified: bool,
    pub verified_date: Option<chrono::NaiveDate>,
    pub wave_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=permohonan)]
pub struct UpdatePermohonan {
    pub user_id: Option<i32>,
    pub company_id: Option<i32>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub verified: Option<bool>,
    pub verified_date: Option<chrono::NaiveDate>,
    pub wave_id: Option<i32>,
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

impl Permohonan {
    pub async fn create(db: &mut Connection, item: &CreatePermohonan) -> QueryResult<Self> {
        use crate::schema::permohonan::dsl::*;

        insert_into(permohonan)
            .values(item)
            .get_result::<Self>(db)
            .await
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<Self>> {
        use crate::schema::permohonan::dsl::*;

        permohonan
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()
    }

    pub async fn paginate_by_user(
        db: &mut Connection,
        param_id: i32,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::permohonan::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = permohonan.count().get_result(db).await?;
        let items = permohonan
            .filter(user_id.eq(param_id))
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

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::permohonan::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = permohonan.count().get_result(db).await?;
        let items = permohonan
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
        item: &UpdatePermohonan,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::permohonan::dsl::*;

        diesel::update(permohonan.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional()
    }

    pub async fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::permohonan::dsl::*;

        diesel::delete(permohonan.filter(id.eq(param_id)))
            .execute(db)
            .await
    }
}
