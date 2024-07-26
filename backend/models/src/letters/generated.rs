/* This file is generated and managed by dsync */
use crate::company::Company;
use crate::diesel::prelude::*;
use crate::log::Log;
use crate::schema::*;
use crate::student::StudentJoined;
use crate::types::{Operation, TableRef};
use crate::user::{User, UserPublic};
use crate::wave::Wave;
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
#[diesel(table_name=letters, primary_key(id), belongs_to(Company, foreign_key=company_id) , belongs_to(User, foreign_key=user_id) , belongs_to(Wave, foreign_key=wave_id))]
pub struct Letter {
    pub id: i32,
    pub user_id: i32,
    pub company_id: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub verified: bool,
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub wave_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LetterBrief {
    pub id: i32,
    pub user: String,
    pub company: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub verified: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct LetterJoined {
    pub id: i32,
    pub user: UserPublic,
    pub company: Company,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub verified: bool,
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub wave: Wave,
    pub students: Vec<StudentJoined>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=letters)]
pub struct CreateLetter {
    pub user_id: Option<i32>,
    pub company_id: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub verified: Option<bool>,
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub wave_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=letters)]
pub struct UpdateLetter {
    pub user_id: Option<i32>,
    pub company_id: Option<i32>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub verified: Option<bool>,
    pub verified_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
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

impl Letter {
    pub async fn get_owner_id(db: &mut Connection, param_id: i32) -> QueryResult<Option<i32>> {
        use crate::schema::letters::dsl::*;

        letters
            .filter(id.eq(param_id))
            .select(user_id)
            .first::<i32>(db)
            .await
            .optional()
    }

    pub async fn create(
        db: &mut Connection,
        item: &CreateLetter,
        param_user_id: i32,
    ) -> QueryResult<Self> {
        use crate::schema::letters::dsl::*;

        let res = diesel::insert_into(letters)
            .values(item)
            .get_result::<Self>(db)
            .await;

        if let Ok(_) = res {
            Log::log(
                db,
                Operation::Create,
                TableRef::Letters,
                param_user_id,
                None::<u8>,
            )
            .await;
        }

        res
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<Self>> {
        use crate::schema::letters::dsl::*;

        letters
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()
    }

    pub async fn read_with_joins(
        db: &mut Connection,
        param_id: i32,
    ) -> QueryResult<Option<LetterJoined>> {
        use crate::letters_student::LettersStudent;
        use crate::schema::class;
        use crate::schema::company;
        use crate::schema::department;
        use crate::schema::letters::dsl::*;
        use crate::schema::student;
        use crate::schema::user;
        use crate::schema::wave;

        let res = letters
            .filter(id.eq(param_id))
            .inner_join(wave::table)
            .inner_join(user::table)
            .inner_join(company::table)
            .first::<(Letter, Wave, User, Company)>(db)
            .await
            .optional()?;
        let Some((item, wave, mut user, company)) = res else {
            return Ok(None);
        };

        let students = LettersStudent::belonging_to(&item)
            .inner_join(student::table.inner_join(class::table.inner_join(department::table)))
            .select((
                student::id,
                student::name,
                class::number,
                department::name,
                student::nis,
            ))
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

        Ok(Some(LetterJoined {
            id: item.id,
            user: user.public(),
            company: company,
            start_date: item.start_date,
            end_date: item.end_date,
            verified: item.verified,
            verified_at: item.verified_at,
            wave: wave,
            students: students,
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
        use crate::schema::letters::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = letters.count().get_result(db).await?;
        let items = match param_user_id {
            Some(n) => {
                letters
                    .filter(user_id.eq(n))
                    .limit(page_size)
                    .offset(page * page_size)
                    .load::<Self>(db)
                    .await?
            }
            None => {
                letters
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
    ) -> QueryResult<PaginationResult<LetterBrief>> {
        use crate::schema::letters::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = letters.count().get_result(db).await?;
        let mut items = match param_user_id {
            Some(n) => {
                letters
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
            }
            None => {
                letters
                    .limit(page_size)
                    .inner_join(user::table)
                    .inner_join(company::table)
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
            }
        };

        let items = items
            .iter_mut()
            .map(
                |(_id, _created_at, _verified, username, company)| LetterBrief {
                    id: *_id,
                    user: mem::take(username),
                    company: mem::take(company),
                    created_at: mem::take(_created_at),
                    verified: *_verified,
                },
            )
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
        item: &UpdateLetter,
        param_user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::letters::dsl::*;

        let previous = Letter::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(letters.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional();

        if let Ok(Some(_)) = res {
            Log::log(
                db,
                Operation::Update,
                TableRef::Letters,
                param_user_id,
                Some(previous),
            )
            .await;
        }

        res
    }

    pub async fn delete(
        db: &mut Connection,
        param_id: i32,
        param_user_id: i32,
    ) -> QueryResult<usize> {
        use crate::schema::letters::dsl::*;

        let previous = Letter::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(letters.filter(id.eq(param_id)))
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
                    TableRef::Letters,
                    param_user_id,
                    Some(previous),
                )
                .await;

                res
            }
            Err(_) => res,
        }
    }

    pub async fn verify_letter(
        db: &mut Connection,
        param_id: i32,
        param_user_id: i32,
    ) -> QueryResult<usize> {
        use crate::schema::letters::dsl::*;

        let previous = Letter::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::update(letters.filter(id.eq(param_id)))
            .set(UpdateLetter {
                user_id: None,
                company_id: None,
                start_date: None,
                end_date: None,
                verified: Some(true),
                verified_at: Some(Some(chrono::Utc::now())),
                wave_id: None,
            })
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
                    TableRef::Letters,
                    param_user_id,
                    Some(previous),
                )
                .await;

                res
            }
            Err(_) => res,
        }
    }

    pub async fn get_letter_order(
        db: &mut Connection,
        letter_id: i32,
        letter_wave_id: i32,
    ) -> QueryResult<u32> {
        use crate::schema::letters::dsl::*;

        let letter = letters
            .filter(verified.eq(true))
            .filter(wave_id.eq(letter_wave_id))
            .order(verified_at.asc())
            .select(id)
            .load::<i32>(db)
            .await?;

        Ok((letter.iter().position(|n| *n == letter_id).unwrap_or(0) as u32) + 1)
    }
}
