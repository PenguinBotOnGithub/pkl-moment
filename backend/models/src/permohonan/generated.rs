/* This file is generated and managed by dsync */

use crate::company::Company;
use crate::log::{CreateLog, Log};
use crate::schema::*;
use crate::types::{Operation, TableRef};
use crate::user::{User, UserPublic};
use crate::wave::Wave;
use diesel::QueryResult;
use diesel::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::error;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermohonanBrief {
    pub id: i32,
    pub user: String,
    pub company: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermohonanJoined {
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
#[diesel(table_name=permohonan)]
pub struct CreatePermohonan {
    pub user_id: Option<i32>,
    pub company_id: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub verified: Option<bool>,
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
    pub verified_date: Option<Option<chrono::NaiveDate>>,
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
    pub async fn get_owner_id(db: &mut Connection, param_id: i32) -> QueryResult<Option<i32>> {
        use crate::schema::permohonan::dsl::*;

        permohonan
            .filter(id.eq(param_id))
            .select(user_id)
            .first::<i32>(db)
            .await
            .optional()
    }

    pub async fn create(
        db: &mut Connection,
        item: &CreatePermohonan,
        param_user_id: i32,
    ) -> QueryResult<Self> {
        use crate::schema::permohonan::dsl::*;

        let res = insert_into(permohonan)
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
                table_affected: TableRef::Permohonan,
                user_id: param_user_id,
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
        use crate::schema::permohonan::dsl::*;

        permohonan
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()
    }

    pub async fn read_with_joins(
        db: &mut Connection,
        param_id: i32,
    ) -> QueryResult<Option<PermohonanJoined>> {
        use crate::permohonan_student::PermohonanStudent;
        use crate::schema::company;
        use crate::schema::permohonan::dsl::*;
        use crate::schema::student;
        use crate::schema::user;
        use crate::schema::wave;

        let res = permohonan
            .filter(id.eq(param_id))
            .inner_join(wave::table)
            .inner_join(user::table)
            .inner_join(company::table)
            .first::<(Permohonan, Wave, User, crate::company::Company)>(db)
            .await
            .optional()?;

        let (item, wave, mut user, company) = match res {
            Some(v) => v,
            None => return Ok(None),
        };

        let students = PermohonanStudent::belonging_to(&item)
            .inner_join(student::table)
            .get_results::<(PermohonanStudent, crate::student::Student)>(db)
            .await
            .optional()?
            .map_or(Vec::new(), |v| v.into_iter().map(|v| v.1).collect());

        Ok(Some(PermohonanJoined {
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

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
        param_user_id: Option<i32>,
    ) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::permohonan::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = permohonan.count().get_result(db).await?;
        let items = match param_user_id {
            Some(n) => {
                permohonan
                    .filter(user_id.eq(n))
                    .limit(page_size)
                    .offset(page * page_size)
                    .load::<Self>(db)
                    .await?
            }
            None => {
                permohonan
                    .limit(page_size)
                    .offset(page * page_size)
                    .load::<Self>(db)
                    .await?
            }
        };

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
        param_user_id: Option<i32>,
    ) -> QueryResult<PaginationResult<PermohonanBrief>> {
        use crate::schema::company;
        use crate::schema::permohonan::dsl::*;
        use crate::schema::user;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = permohonan.count().get_result(db).await?;
        let items = match param_user_id {
            Some(n) => permohonan
                .filter(user_id.eq(n))
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
                .map(|v| PermohonanBrief {
                    id: v.0,
                    user: v.3,
                    company: v.4,
                    created_at: v.1,
                    verified: v.2,
                })
                .collect(),
            None => permohonan
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
                .map(|v| PermohonanBrief {
                    id: v.0,
                    user: v.3,
                    company: v.4,
                    created_at: v.1,
                    verified: v.2,
                })
                .collect(),
        };

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
        param_user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::permohonan::dsl::*;

        let previous = permohonan
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(permohonan.filter(id.eq(param_id)))
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
                table_affected: TableRef::Permohonan,
                user_id: param_user_id,
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

    pub async fn delete(
        db: &mut Connection,
        param_id: i32,
        param_user_id: i32,
    ) -> QueryResult<usize> {
        use crate::schema::permohonan::dsl::*;

        let previous = permohonan
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(permohonan.filter(id.eq(param_id)))
            .execute(db)
            .await;

        let Ok(_) = res.as_ref() else {
            return res;
        };

        if let Err(e) = Log::create(
            db,
            &CreateLog {
                operation_type: Operation::Delete,
                table_affected: TableRef::Permohonan,
                user_id: param_user_id,
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

    pub async fn verify(
        db: &mut Connection,
        param_id: i32,
        param_user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::permohonan::dsl::*;

        let previous = permohonan
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(permohonan.filter(id.eq(param_id)))
            .set(UpdatePermohonan {
                user_id: None,
                company_id: None,
                start_date: None,
                end_date: None,
                verified: Some(true),
                verified_date: Some(Some(chrono::Utc::now().date_naive())),
                wave_id: None,
            })
            .get_result(db)
            .await
            .optional();

        let Ok(_) = res.as_ref() else {
            return res;
        };

        if let Err(e) = Log::create(
            db,
            &CreateLog {
                operation_type: Operation::Verify,
                table_affected: TableRef::Permohonan,
                user_id: param_user_id,
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

    pub async fn unverify(
        db: &mut Connection,
        param_id: i32,
        param_user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::permohonan::dsl::*;

        let previous = permohonan
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(permohonan.filter(id.eq(param_id)))
            .set(UpdatePermohonan {
                user_id: None,
                company_id: None,
                start_date: None,
                end_date: None,
                verified: Some(false),
                verified_date: Some(None),
                wave_id: None,
            })
            .get_result(db)
            .await
            .optional();

        let Ok(_) = res.as_ref() else {
            return res;
        };

        if let Err(e) = Log::create(
            db,
            &CreateLog {
                operation_type: Operation::Unverify,
                table_affected: TableRef::Permohonan,
                user_id: param_user_id,
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

    pub async fn get_letter_order(
        db: &mut Connection,
        letter_id: i32,
        letter_wave_id: i32,
    ) -> QueryResult<u32> {
        use crate::schema::permohonan::dsl::*;

        let letters = permohonan
            .filter(verified.eq(true))
            .filter(wave_id.eq(letter_wave_id))
            .order(verified_date.asc())
            .select(id)
            .load::<i32>(db)
            .await?;

        Ok((letters.iter().position(|n| *n == letter_id).unwrap_or(0) as u32) + 1)
    }
}
