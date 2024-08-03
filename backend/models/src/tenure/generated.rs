/* This file is generated and managed by dsync */
use crate::diesel::prelude::*;
use crate::letters::{Letter, LetterBrief};
use crate::log::Log;
use crate::schema::*;
use crate::student::{Student, StudentJoined};
use crate::types::UserRole;
use crate::types::{Operation, TableRef};
use crate::user::{User, UserPublic};
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
#[diesel(table_name=tenure, primary_key(id), belongs_to(Letter, foreign_key=letters_id) , belongs_to(Student, foreign_key=student_id))]
pub struct Tenure {
    pub id: i32,
    pub student_id: i32,
    pub advsch_id: Option<i32>,
    pub advdudi_id: Option<i32>,
    pub letters_id: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct TenureJoined {
    pub id: i32,
    pub student: StudentJoined,
    pub advisor_sch: Option<UserPublic>,
    pub advisor_dudi: Option<UserPublic>,
    pub letter: LetterBrief,
}

#[derive(Debug, Clone, Serialize)]
pub struct TenureBrief {
    pub id: i32,
    pub student: String,
    pub advisor_sch: Option<String>,
    pub advisor_dudi: Option<String>,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Debug, Clone, Serialize)]
pub struct TenureMini {
    pub id: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=tenure)]
pub struct CreateTenure {
    pub student_id: i32,
    pub advsch_id: Option<i32>,
    pub advdudi_id: Option<i32>,
    pub letters_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=tenure)]
pub struct UpdateTenure {
    pub student_id: Option<i32>,
    pub advsch_id: Option<Option<i32>>,
    pub advdudi_id: Option<Option<i32>>,
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

impl Tenure {
    pub async fn create(
        db: &mut Connection,
        item: &CreateTenure,
        param_user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::letters;
        use crate::schema::tenure::dsl::*;
        use crate::schema::user;

        if let Some(n) = item.advsch_id {
            let role = user::dsl::user
                .filter(user::dsl::id.eq(n))
                .select(user::role)
                .first::<UserRole>(db)
                .await
                .optional()?;

            match role {
                None => return Err(diesel::result::Error::NotFound),
                Some(UserRole::AdvisorSchool) => {}
                _ => return Ok(None),
            }
        }

        if let Some(n) = item.advdudi_id {
            let role = user::dsl::user
                .filter(user::dsl::id.eq(n))
                .select(user::role)
                .first::<UserRole>(db)
                .await
                .optional()?;

            match role {
                None => return Err(diesel::result::Error::NotFound),
                Some(UserRole::AdvisorSchool) => {}
                _ => return Ok(None),
            }
        }

        let letters_verified = letters::dsl::letters
            .filter(letters::dsl::id.eq(item.letters_id))
            .select(letters::verified)
            .first::<bool>(db)
            .await
            .optional()?;
        match letters_verified {
            None => return Err(diesel::result::Error::NotFound),
            Some(v) => {
                if !v {
                    return Ok(None);
                }
            }
        }

        let res = diesel::insert_into(tenure)
            .values(item)
            .get_result::<Self>(db)
            .await;

        if let Ok(_) = res {
            Log::log(
                db,
                Operation::Create,
                TableRef::Tenure,
                param_user_id,
                None::<u8>,
            )
            .await;
        }

        match res {
            Ok(v) => Ok(Some(v)),
            Err(e) => Err(e),
        }
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<Tenure>> {
        use crate::schema::tenure::dsl::*;

        tenure
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()
    }

    pub async fn read_joined(
        db: &mut Connection,
        param_id: i32,
    ) -> QueryResult<Option<TenureJoined>> {
        use crate::schema::company;
        use crate::schema::letters;
        use crate::schema::tenure::dsl::*;
        use crate::schema::user;

        let res = tenure
            .inner_join(
                letters::table
                    .inner_join(user::table)
                    .inner_join(company::table),
            )
            .filter(id.eq(param_id))
            .select((
                tenure::all_columns(),
                letters::id,
                letters::created_at,
                letters::verified,
                user::username,
                company::name,
            ))
            .first::<(
                Self,
                i32,
                chrono::DateTime<chrono::Utc>,
                bool,
                String,
                String,
            )>(db)
            .await
            .optional()?;
        let Some((t_tenure, l_id, l_cts, l_verified, u_username, cname)) = res else {
            return Ok(None);
        };

        let adv_sch = match t_tenure.advsch_id {
            Some(n) => {
                let user = User::read(db, n).await?;
                let Some(mut user) = user else {
                    return Ok(None);
                };

                Some(user.public())
            }
            None => None,
        };

        let adv_dudi = match t_tenure.advdudi_id {
            Some(n) => {
                let user = User::read(db, n).await?;
                let Some(mut user) = user else {
                    return Ok(None);
                };

                Some(user.public())
            }
            None => None,
        };

        let student = Student::read(db, t_tenure.student_id).await?;
        let Some(student) = student else {
            return Ok(None);
        };

        Ok(Some(TenureJoined {
            id: t_tenure.id,
            student: student,
            advisor_sch: adv_sch,
            advisor_dudi: adv_dudi,
            letter: LetterBrief {
                id: l_id,
                user: u_username,
                company: cname,
                created_at: l_cts,
                verified: l_verified,
            },
        }))
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<TenureBrief>> {
        use crate::schema::letters;
        use crate::schema::student;
        use crate::schema::tenure::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = tenure.count().get_result(db).await?;
        let items = tenure
            .limit(page_size)
            .offset(page * page_size)
            .inner_join(student::table)
            .inner_join(letters::table)
            .select((
                id,
                student::name,
                advsch_id,
                advdudi_id,
                letters::start_date,
                letters::end_date,
            ))
            .load::<(
                i32,
                String,
                Option<i32>,
                Option<i32>,
                chrono::NaiveDate,
                chrono::NaiveDate,
            )>(db)
            .await?;

        let mut items_brief = Vec::new();
        for (t_id, mut s_name, t_advs, t_advd, s_date, e_date) in items {
            let advs = match t_advs {
                Some(n) => user::dsl::user
                    .filter(user::dsl::id.eq(n))
                    .select(user::username)
                    .first::<String>(db)
                    .await
                    .optional()?,
                None => None,
            };

            let advd = match t_advd {
                Some(n) => user::dsl::user
                    .filter(user::dsl::id.eq(n))
                    .select(user::username)
                    .first::<String>(db)
                    .await
                    .optional()?,
                None => None,
            };

            items_brief.push(TenureBrief {
                id: t_id,
                student: mem::take(&mut s_name),
                advisor_sch: advs,
                advisor_dudi: advd,
                start_date: s_date,
                end_date: e_date,
            });
        }

        Ok(PaginationResult {
            items: items_brief,
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
        item: &UpdateTenure,
        param_user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::tenure::dsl::*;

        let previous = Tenure::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(tenure.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional();

        if let Ok(Some(_)) = res {
            Log::log(
                db,
                Operation::Update,
                TableRef::Tenure,
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
        use crate::schema::tenure::dsl::*;

        let previous = Tenure::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(tenure.filter(id.eq(param_id)))
            .execute(db)
            .await;

        if let Ok(n) = res {
            if n == 0 {
                return res;
            }

            Log::log(
                db,
                Operation::Delete,
                TableRef::Tenure,
                param_user_id,
                Some(previous),
            )
            .await;
        }

        res
    }

    pub async fn add_advisor_sch(
        db: &mut Connection,
        param_id: i32,
        param_advisor_id: i32,
        param_user_id: i32,
    ) -> QueryResult<isize> {
        use crate::schema::tenure::dsl::*;
        use crate::schema::user;

        let previous = tenure
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let adv = user::dsl::user
            .filter(user::dsl::id.eq(param_advisor_id))
            .select(user::role)
            .first::<UserRole>(db)
            .await
            .optional()?;
        let Some(adv) = adv else {
            return Ok(0);
        };

        match adv {
            UserRole::AdvisorSchool => {}
            _ => return Ok(-1),
        }

        let res = diesel::update(tenure.filter(id.eq(param_id)))
            .set(UpdateTenure {
                student_id: None,
                advsch_id: Some(Some(param_advisor_id)),
                advdudi_id: None,
            })
            .execute(db)
            .await
            .map(|e| e as isize);

        if let Ok(n) = res {
            if n == 0 {
                return res;
            }

            Log::log(
                db,
                Operation::Update,
                TableRef::Tenure,
                param_user_id,
                Some(previous),
            )
            .await;
        }

        res
    }

    pub async fn add_advisor_dudi(
        db: &mut Connection,
        param_id: i32,
        param_advisor_id: i32,
        param_user_id: i32,
    ) -> QueryResult<isize> {
        use crate::schema::tenure::dsl::*;
        use crate::schema::user;

        let previous = tenure
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let adv = user::dsl::user
            .filter(user::dsl::id.eq(param_advisor_id))
            .select(user::role)
            .first::<UserRole>(db)
            .await
            .optional()?;
        let Some(adv) = adv else {
            return Ok(0);
        };

        match adv {
            UserRole::AdvisorDudi => {}
            _ => return Ok(-1),
        }

        let res = diesel::update(tenure.filter(id.eq(param_id)))
            .set(UpdateTenure {
                student_id: None,
                advsch_id: None,
                advdudi_id: Some(Some(param_advisor_id)),
            })
            .execute(db)
            .await
            .map(|e| e as isize);

        if let Ok(n) = res {
            if n == 0 {
                return res;
            }

            Log::log(
                db,
                Operation::Update,
                TableRef::Tenure,
                param_user_id,
                Some(previous),
            )
            .await;
        }

        res
    }

    pub async fn remove_advisor_sch(
        db: &mut Connection,
        param_id: i32,
        param_user_id: i32,
    ) -> QueryResult<usize> {
        use crate::schema::tenure::dsl::*;

        let previous = Tenure::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::update(tenure.filter(id.eq(param_id)))
            .set(UpdateTenure {
                student_id: None,
                advsch_id: Some(None),
                advdudi_id: None,
            })
            .execute(db)
            .await;

        if let Ok(n) = res {
            if n != 0 {
                Log::log(
                    db,
                    Operation::Update,
                    TableRef::Tenure,
                    param_user_id,
                    Some(previous),
                )
                .await;
            }
        }

        res
    }

    pub async fn remove_advisor_dudi(
        db: &mut Connection,
        param_id: i32,
        param_user_id: i32,
    ) -> QueryResult<usize> {
        use crate::schema::tenure::dsl::*;

        let previous = Tenure::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::update(tenure.filter(id.eq(param_id)))
            .set(UpdateTenure {
                student_id: None,
                advsch_id: None,
                advdudi_id: Some(None),
            })
            .execute(db)
            .await;

        if let Ok(n) = res {
            if n != 0 {
                Log::log(
                    db,
                    Operation::Update,
                    TableRef::Tenure,
                    param_user_id,
                    Some(previous),
                )
                .await;
            }
        }

        res
    }

    pub async fn get_tenures_by_user(
        db: &mut Connection,
        param_user_id: i32,
    ) -> QueryResult<Vec<TenureMini>> {
        use crate::schema::letters;
        use crate::schema::student;
        use crate::schema::tenure::dsl::*;
        use crate::schema::user;

        let res = tenure
            .inner_join(student::table.inner_join(user::table))
            .inner_join(letters::table)
            .filter(user::id.eq(param_user_id))
            .select((id, letters::start_date, letters::end_date))
            .load::<(i32, chrono::NaiveDate, chrono::NaiveDate)>(db)
            .await?
            .iter()
            .map(|v| TenureMini {
                id: v.0,
                start_date: v.1,
                end_date: v.2,
            })
            .collect();

        Ok(res)
    }
}
