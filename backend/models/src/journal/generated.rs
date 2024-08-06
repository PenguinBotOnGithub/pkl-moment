/* This file is generated and managed by dsync */
use anyhow::anyhow;
use diesel::QueryResult;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::mem;

use crate::diesel::prelude::*;
use crate::log::Log;
use crate::schema::journal;
use crate::tenure::Tenure;
use crate::types::{Operation, TableRef};

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
#[diesel(table_name=journal, primary_key(id), belongs_to(Tenure, foreign_key=tenure_id))]
pub struct Journal {
    pub id: i32,
    pub tenure_id: i32,
    pub division: String,
    pub entry_date: chrono::NaiveDate,
    pub start_time: chrono::NaiveTime,
    pub end_time: chrono::NaiveTime,
    pub activity: String,
    pub img_url: String,
    pub extra: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct JournalJoined {
    pub id: i32,
    pub student: String,
    pub company: String,
    pub division: String,
    pub entry_date: chrono::NaiveDate,
    pub start_time: chrono::NaiveTime,
    pub end_time: chrono::NaiveTime,
    pub activity: String,
    pub img_url: String,
    pub extra: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=journal)]
pub struct CreateJournal {
    pub tenure_id: i32,
    pub division: String,
    pub entry_date: chrono::NaiveDate,
    pub start_time: chrono::NaiveTime,
    pub end_time: chrono::NaiveTime,
    pub activity: String,
    pub img_url: String,
    pub extra: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=journal)]
pub struct UpdateJournal {
    pub division: Option<String>,
    pub entry_date: Option<chrono::NaiveDate>,
    pub start_time: Option<chrono::NaiveTime>,
    pub end_time: Option<chrono::NaiveTime>,
    pub activity: Option<String>,
    pub img_url: Option<String>,
    pub extra: Option<Option<String>>,
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

impl JournalJoined {
    pub fn new(
        j_id: i32,
        mut s_name: String,
        mut c_name: String,
        mut j_div: String,
        e_date: chrono::NaiveDate,
        s_time: chrono::NaiveTime,
        e_time: chrono::NaiveTime,
        mut j_act: String,
        mut j_img: String,
        j_extra: Option<String>,
        j_cat: chrono::DateTime<chrono::Utc>,
        j_uat: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        JournalJoined {
            id: j_id,
            student: mem::take(&mut s_name),
            company: mem::take(&mut c_name),
            division: mem::take(&mut j_div),
            entry_date: e_date,
            start_time: s_time,
            end_time: e_time,
            activity: mem::take(&mut j_act),
            img_url: mem::take(&mut j_img),
            extra: j_extra,
            created_at: j_cat,
            updated_at: j_uat,
        }
    }
}

impl Journal {
    pub async fn get_owner_id(db: &mut Connection, param_id: i32) -> QueryResult<Option<i32>> {
        use crate::schema::journal::dsl::*;
        use crate::schema::student;
        use crate::schema::tenure;
        use crate::schema::user;

        journal
            .filter(id.eq(param_id))
            .inner_join(tenure::table.inner_join(student::table.inner_join(user::table)))
            .select(user::id)
            .first::<i32>(db)
            .await
            .optional()
    }

    pub async fn get_advisors(
        db: &mut Connection,
        param_id: i32,
    ) -> QueryResult<Option<(Option<i32>, Option<i32>)>> {
        use crate::schema::journal::dsl::*;
        use crate::schema::tenure;

        journal
            .filter(id.eq(param_id))
            .inner_join(tenure::table)
            .select((tenure::advsch_id, tenure::advdudi_id))
            .first::<(Option<i32>, Option<i32>)>(db)
            .await
            .optional()
    }

    pub async fn create(
        db: &mut Connection,
        item: &CreateJournal,
        param_user_id: i32,
    ) -> QueryResult<Self> {
        use crate::schema::journal::dsl::*;

        let res = diesel::insert_into(journal)
            .values(item)
            .get_result::<Self>(db)
            .await;

        if let Ok(_) = res {
            Log::log(
                db,
                Operation::Create,
                TableRef::Journal,
                param_user_id,
                None::<u8>,
            )
            .await;
        }

        res
    }

    pub async fn create_checked(
        db: &mut Connection,
        item: &CreateJournal,
        param_user_id: i32,
    ) -> anyhow::Result<Journal> {
        use crate::schema::journal::dsl::*;
        use crate::schema::letters;
        use crate::schema::tenure::dsl as tenure;

        let res = tenure::tenure
            .filter(tenure::id.eq(item.tenure_id))
            .inner_join(letters::table)
            .select((
                crate::schema::tenure::all_columns,
                letters::verified,
                letters::start_date,
                letters::end_date,
            ))
            .first::<(Tenure, bool, chrono::NaiveDate, chrono::NaiveDate)>(db)
            .await
            .optional()?;
        let Some(res) = res else {
            return Err(anyhow!("tenure data not found"));
        };
        let (tenure, verified, s_date, e_date) = res;

        if let None = tenure.advsch_id {
            return Err(anyhow!(
                "not allowed to create journal when an advisor from school has not been assigned"
            ));
        }

        if let None = tenure.advdudi_id {
            return Err(anyhow!(
                "not allowed to create journal when an advisor from dudi has not been assigned"
            ));
        }

        if !verified {
            return Err(anyhow!("letters data is not verified"));
        }

        let today = chrono::Local::now().date_naive();
        if s_date > today {
            return Err(anyhow!("user's tenure has not started yet"));
        }

        if item.entry_date < s_date {
            return Err(anyhow!(
                "entry_date can not be earlier than tenure's start date"
            ));
        }
        if item.entry_date > e_date {
            return Err(anyhow!(
                "entry_date can not be later than tenure's end date"
            ));
        }

        let res = diesel::insert_into(journal)
            .values(item)
            .get_result::<Self>(db)
            .await?;

        Log::log(
            db,
            Operation::Create,
            TableRef::Journal,
            param_user_id,
            None::<u8>,
        )
        .await;

        Ok(res)
    }

    pub async fn read_joined(
        db: &mut Connection,
        param_id: i32,
    ) -> QueryResult<Option<JournalJoined>> {
        use crate::schema::company;
        use crate::schema::journal::dsl::*;
        use crate::schema::letters;
        use crate::schema::student;
        use crate::schema::tenure;

        let res = journal
            .filter(id.eq(param_id))
            .inner_join(
                tenure::table
                    .inner_join(student::table)
                    .inner_join(letters::table.inner_join(company::table)),
            )
            .select((
                id,
                student::name,
                company::name,
                division,
                entry_date,
                start_time,
                end_time,
                activity,
                img_url,
                extra,
                created_at,
                updated_at,
            ))
            .first::<(
                i32,
                String,
                String,
                String,
                chrono::NaiveDate,
                chrono::NaiveTime,
                chrono::NaiveTime,
                String,
                String,
                Option<String>,
                chrono::DateTime<chrono::Utc>,
                chrono::DateTime<chrono::Utc>,
            )>(db)
            .await
            .optional()?;

        let Some((a, b, c, d, e, f, g, h, i, j, k, l)) = res else {
            return Ok(None);
        };

        Ok(Some(JournalJoined::new(a, b, c, d, e, f, g, h, i, j, k, l)))
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<JournalJoined>> {
        use crate::schema::company;
        use crate::schema::journal::dsl::*;
        use crate::schema::letters;
        use crate::schema::student;
        use crate::schema::tenure;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = journal.count().get_result(db).await?;
        let items = journal
            .limit(page_size)
            .offset(page * page_size)
            .order(created_at.desc())
            .inner_join(
                tenure::table
                    .inner_join(student::table)
                    .inner_join(letters::table.inner_join(company::table)),
            )
            .select((
                id,
                student::name,
                company::name,
                division,
                entry_date,
                start_time,
                end_time,
                activity,
                img_url,
                extra,
                created_at,
                updated_at,
            ))
            .load::<(
                i32,
                String,
                String,
                String,
                chrono::NaiveDate,
                chrono::NaiveTime,
                chrono::NaiveTime,
                String,
                String,
                Option<String>,
                chrono::DateTime<chrono::Utc>,
                chrono::DateTime<chrono::Utc>,
            )>(db)
            .await?
            .into_iter()
            .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
                JournalJoined::new(a, b, c, d, e, f, g, h, i, j, k, l)
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

    pub async fn paginate_by_student(
        db: &mut Connection,
        page: i64,
        page_size: i64,
        param_user_id: i32,
    ) -> QueryResult<PaginationResult<JournalJoined>> {
        use crate::schema::company;
        use crate::schema::journal::dsl::*;
        use crate::schema::letters;
        use crate::schema::student;
        use crate::schema::tenure;
        use crate::schema::user;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = journal.count().get_result(db).await?;
        let items = journal
            .limit(page_size)
            .offset(page * page_size)
            .order(created_at.desc())
            .inner_join(
                tenure::table
                    .inner_join(student::table.inner_join(user::table))
                    .inner_join(letters::table.inner_join(company::table)),
            )
            .filter(user::id.eq(param_user_id))
            .select((
                id,
                student::name,
                company::name,
                division,
                entry_date,
                start_time,
                end_time,
                activity,
                img_url,
                extra,
                created_at,
                updated_at,
            ))
            .load::<(
                i32,
                String,
                String,
                String,
                chrono::NaiveDate,
                chrono::NaiveTime,
                chrono::NaiveTime,
                String,
                String,
                Option<String>,
                chrono::DateTime<chrono::Utc>,
                chrono::DateTime<chrono::Utc>,
            )>(db)
            .await?
            .into_iter()
            .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
                JournalJoined::new(a, b, c, d, e, f, g, h, i, j, k, l)
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

    pub async fn paginate_by_advisor(
        db: &mut Connection,
        page: i64,
        page_size: i64,
        param_user_id: i32,
    ) -> QueryResult<PaginationResult<JournalJoined>> {
        use crate::schema::company;
        use crate::schema::journal::dsl::*;
        use crate::schema::letters;
        use crate::schema::student;
        use crate::schema::tenure;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = journal.count().get_result(db).await?;
        let items = journal
            .limit(page_size)
            .offset(page * page_size)
            .order(created_at.desc())
            .inner_join(
                tenure::table
                    .inner_join(student::table)
                    .inner_join(letters::table.inner_join(company::table)),
            )
            .filter(
                tenure::advsch_id
                    .eq(param_user_id)
                    .or(tenure::advdudi_id.eq(param_user_id)),
            )
            .select((
                id,
                student::name,
                company::name,
                division,
                entry_date,
                start_time,
                end_time,
                activity,
                img_url,
                extra,
                created_at,
                updated_at,
            ))
            .load::<(
                i32,
                String,
                String,
                String,
                chrono::NaiveDate,
                chrono::NaiveTime,
                chrono::NaiveTime,
                String,
                String,
                Option<String>,
                chrono::DateTime<chrono::Utc>,
                chrono::DateTime<chrono::Utc>,
            )>(db)
            .await?
            .into_iter()
            .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
                JournalJoined::new(a, b, c, d, e, f, g, h, i, j, k, l)
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
        item: &UpdateJournal,
        param_user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::journal::dsl::*;

        let previous = journal
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(journal.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional();

        if let Ok(Some(_)) = res {
            Log::log(
                db,
                Operation::Update,
                TableRef::Journal,
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
        use crate::schema::journal::dsl::*;

        let previous = journal
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(journal.filter(id.eq(param_id)))
            .execute(db)
            .await;

        if let Ok(n) = res {
            if n > 0 {
                Log::log(
                    db,
                    Operation::Delete,
                    TableRef::Journal,
                    param_user_id,
                    Some(previous),
                )
                .await;
            }
        }

        res
    }

    pub async fn return_tenure(db: &mut Connection, param_id: i32) -> QueryResult<Option<Tenure>> {
        use crate::schema::journal::dsl::*;
        use crate::schema::tenure;

        journal
            .filter(id.eq(param_id))
            .inner_join(tenure::table)
            .select(tenure::all_columns)
            .first::<Tenure>(db)
            .await
            .optional()
    }
}
