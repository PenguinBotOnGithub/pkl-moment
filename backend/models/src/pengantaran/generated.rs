/* This file is generated and managed by dsync */

use diesel::QueryResult;
use diesel::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::company::Company;
use crate::schema::*;
use crate::user::{User, UserPublic};
use crate::wave::Wave;

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
#[diesel(table_name=pengantaran, primary_key(id), belongs_to(Company, foreign_key=company_id) , belongs_to(User, foreign_key=user_id) , belongs_to(Wave, foreign_key=wave_id))]
pub struct Pengantaran {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PengantaranBrief {
    pub id: i32,
    pub user: String,
    pub company: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PengantaranJoined {
    pub id: i32,
    pub user: UserPublic,
    pub company: Company,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub verified: bool,
    pub verified_date: Option<chrono::NaiveDate>,
    pub wave: Wave,
    pub students: Vec<crate::student::Student>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=pengantaran)]
pub struct CreatePengantaran {
    pub user_id: Option<i32>,
    pub company_id: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub verified: Option<bool>,
    pub verified_date: Option<chrono::NaiveDate>,
    pub wave_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=pengantaran)]
pub struct UpdatePengantaran {
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

impl Pengantaran {
    pub async fn create(db: &mut Connection, item: &CreatePengantaran) -> QueryResult<Self> {
        use crate::schema::pengantaran::dsl::*;

        insert_into(pengantaran)
            .values(item)
            .get_result::<Self>(db)
            .await
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<Self>> {
        use crate::schema::pengantaran::dsl::*;

        pengantaran
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()
    }

    pub async fn read_with_joins(
        db: &mut Connection,
        param_id: i32,
    ) -> QueryResult<Option<PengantaranJoined>> {
        use crate::pengantaran_student::PengantaranStudent;
        use crate::schema::company;
        use crate::schema::pengantaran::dsl::*;
        use crate::schema::student;
        use crate::schema::user;
        use crate::schema::wave;

        let res = pengantaran
            .filter(id.eq(param_id))
            .inner_join(wave::table)
            .inner_join(user::table)
            .inner_join(company::table)
            .first::<(Pengantaran, Wave, User, Company)>(db)
            .await
            .optional()?;

        let (item, wave, mut user, company) = match res {
            Some(v) => v,
            None => return Ok(None),
        };

        let students = PengantaranStudent::belonging_to(&item)
            .inner_join(student::table)
            .get_results::<(PengantaranStudent, crate::student::Student)>(db)
            .await
            .optional()?
            .map_or(Vec::new(), |v| v.into_iter().map(|v| v.1).collect());

        Ok(Some(PengantaranJoined {
            id: item.id,
            user: user.public(),
            company,
            start_date: item.start_date,
            end_date: item.end_date,
            verified: item.verified,
            verified_date: item.verified_date,
            wave,
            students,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }))
    }

    pub async fn paginate_by_user(
        db: &mut Connection,
        param_id: i32,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::pengantaran::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = pengantaran.count().get_result(db).await?;
        let items = pengantaran
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

    pub async fn paginate_brief_by_user(
        db: &mut Connection,
        param_id: i32,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<PengantaranBrief>> {
        use crate::schema::company;
        use crate::schema::pengantaran::dsl::*;
        use crate::schema::user;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = pengantaran.count().get_result(db).await?;
        let items = pengantaran
            .filter(user_id.eq(param_id))
            .inner_join(user::table)
            .inner_join(company::table)
            .limit(page_size)
            .offset(page * page_size)
            .select((
                id,
                created_at,
                verified,
                user::dsl::username,
                company::dsl::name,
            ))
            .load::<(i32, chrono::DateTime<chrono::Utc>, bool, String, String)>(db)
            .await?
            .into_iter()
            .map(|v| PengantaranBrief {
                id: v.0,
                user: v.3,
                company: v.4,
                created_at: v.1,
                verified: v.2,
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

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::pengantaran::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = pengantaran.count().get_result(db).await?;
        let items = pengantaran
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

    pub async fn paginate_brief(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<PengantaranBrief>> {
        use crate::schema::company;
        use crate::schema::pengantaran::dsl::*;
        use crate::schema::user;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = pengantaran.count().get_result(db).await?;
        let items = pengantaran
            .inner_join(user::table)
            .inner_join(company::table)
            .limit(page_size)
            .offset(page * page_size)
            .select((
                id,
                created_at,
                verified,
                user::dsl::username,
                company::dsl::name,
            ))
            .load::<(i32, chrono::DateTime<chrono::Utc>, bool, String, String)>(db)
            .await?
            .into_iter()
            .map(|v| PengantaranBrief {
                id: v.0,
                user: v.3,
                company: v.4,
                created_at: v.1,
                verified: v.2,
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
        item: &UpdatePengantaran,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::pengantaran::dsl::*;

        diesel::update(pengantaran.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional()
    }

    pub async fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::pengantaran::dsl::*;

        diesel::delete(pengantaran.filter(id.eq(param_id)))
            .execute(db)
            .await
    }
}
