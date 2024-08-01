/* This file is generated and managed by dsync */
use crate::class::ClassJoined;
use crate::company::Company;
use crate::diesel::prelude::*;
use crate::log::Log;
use crate::schema::*;
use crate::student::{Student, StudentJoined};
use crate::types::{Operation, TableRef};
use crate::user::User;
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
#[diesel(table_name=journal, primary_key(id), belongs_to(Company, foreign_key=company_id) , belongs_to(Student, foreign_key=student_id))]
pub struct Journal {
    pub id: i32,
    pub student_id: i32,
    pub company_id: i32,
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
    pub student: StudentJoined,
    pub company: Company,
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
    pub student_id: i32,
    pub company_id: i32,
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

impl Journal {
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

    pub async fn read_joined(
        db: &mut Connection,
        param_id: i32,
    ) -> QueryResult<Option<JournalJoined>> {
        use crate::schema::class;
        use crate::schema::company;
        use crate::schema::department;
        use crate::schema::journal::dsl::*;
        use crate::schema::student;
        use crate::schema::user;

        let res = journal
            .filter(id.eq(param_id))
            .inner_join(
                student::table
                    .inner_join(class::table.inner_join(department::table))
                    .inner_join(user::table),
            )
            .inner_join(company::table)
            .select((
                id,
                student::id,
                student::name,
                class::id,
                class::grade,
                class::number,
                department::name,
                student::nis,
                division,
                entry_date,
                start_time,
                end_time,
                activity,
                img_url,
                extra,
                created_at,
                updated_at,
                company::all_columns,
                user::all_columns,
            ))
            .first::<(
                i32,
                i32,
                String,
                i32,
                i32,
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
                Company,
                User,
            )>(db)
            .await
            .optional()?;

        let Some((
            j_id,
            s_id,
            mut s_name,
            c_id,
            c_grade,
            c_num,
            d_name,
            mut s_nis,
            mut d_string,
            e_date,
            s_time,
            e_time,
            mut a_string,
            mut i_url,
            extra_info,
            c_ts,
            u_ts,
            company,
            mut user,
        )) = res
        else {
            return Ok(None);
        };

        Ok(Some(JournalJoined {
            id: j_id,
            student: StudentJoined {
                id: s_id,
                name: mem::take(&mut s_name),
                class: ClassJoined {
                    id: c_id,
                    grade: c_grade,
                    number: c_num,
                    department: d_name,
                },
                nis: mem::take(&mut s_nis),
                user: user.public(),
            },
            company: company,
            division: mem::take(&mut d_string),
            entry_date: e_date,
            start_time: s_time,
            end_time: e_time,
            activity: mem::take(&mut a_string),
            img_url: mem::take(&mut i_url),
            extra: extra_info,
            created_at: c_ts,
            updated_at: u_ts,
        }))
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::journal::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = journal.count().get_result(db).await?;
        let items = journal
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
}
