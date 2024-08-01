/* This file is generated and managed by dsync */
use crate::class::ClassJoined;
use crate::diesel::prelude::*;
use crate::letters::Letter;
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
#[diesel(table_name=letters_student, primary_key(id), belongs_to(Letter, foreign_key=letters_id) , belongs_to(Student, foreign_key=student_id))]
pub struct LettersStudent {
    pub id: i32,
    pub letters_id: i32,
    pub student_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=letters_student)]
pub struct CreateLettersStudent {
    pub letters_id: i32,
    pub student_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=letters_student)]
pub struct UpdateLettersStudent {
    pub letters_id: Option<i32>,
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

impl LettersStudent {
    pub async fn create(
        db: &mut Connection,
        item: &CreateLettersStudent,
        user_id: i32,
    ) -> QueryResult<Self> {
        use crate::schema::letters_student::dsl::*;

        let res = diesel::insert_into(letters_student)
            .values(item)
            .get_result::<Self>(db)
            .await;

        if let Ok(_) = res {
            Log::log(
                db,
                Operation::Create,
                TableRef::LettersStudent,
                user_id,
                None::<u8>,
            )
            .await;
        }

        res
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<Self>> {
        use crate::schema::letters_student::dsl::*;

        letters_student
            .filter(id.eq(param_id))
            .first::<Self>(db)
            .await
            .optional()
    }

    pub async fn filter_by_letter(
        db: &mut Connection,
        param_letter_id: i32,
    ) -> QueryResult<Option<Vec<StudentJoined>>> {
        use crate::schema::class;
        use crate::schema::department;
        use crate::schema::letters_student::dsl::*;
        use crate::schema::student;
        use crate::schema::user;

        let res = letters_student
            .filter(letters_id.eq(param_letter_id))
            .inner_join(
                student::table
                    .inner_join(class::table.inner_join(department::table))
                    .inner_join(user::table),
            )
            .select((
                student::id,
                student::name,
                class::id,
                class::grade,
                class::number,
                department::name,
                student::nis,
                user::all_columns,
            ))
            .load::<(i32, String, i32, i32, i32, String, String, User)>(db)
            .await
            .optional()?;

        match res {
            None => Ok(None),
            Some(mut v) => {
                let constructed = v
                    .iter_mut()
                    .map(
                        |(
                            ref s_id,
                            ref mut name,
                            ref c_id,
                            ref grade,
                            ref number,
                            ref mut d_name,
                            ref mut nis,
                            ref mut user,
                        )| StudentJoined {
                            id: *s_id,
                            name: mem::take(name),
                            class: ClassJoined {
                                id: *c_id,
                                grade: *grade,
                                number: *number,
                                department: mem::take(d_name),
                            },
                            nis: mem::take(nis),
                            user: user.public(),
                        },
                    )
                    .collect();

                Ok(Some(constructed))
            }
        }
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::letters_student::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = letters_student.count().get_result(db).await?;
        let items = letters_student
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

    pub async fn delete(db: &mut Connection, param_id: i32, user_id: i32) -> QueryResult<usize> {
        use crate::schema::letters_student::dsl::*;

        let previous = LettersStudent::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(letters_student.filter(id.eq(param_id)))
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
                    TableRef::LettersStudent,
                    user_id,
                    Some(previous),
                )
                .await;

                res
            }
            Err(_) => res,
        }
    }

    pub async fn delete_by_student_letter_id(
        db: &mut Connection,
        param_student_id: i32,
        param_letter_id: i32,
        param_user_id: i32,
    ) -> QueryResult<usize> {
        use crate::schema::letters_student::dsl::*;

        let previous = letters_student
            .filter(student_id.eq(param_student_id))
            .filter(letters_id.eq(param_letter_id))
            .first::<Self>(db)
            .await
            .optional()?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(
            letters_student
                .filter(student_id.eq(param_student_id))
                .filter(letters_id.eq(param_letter_id)),
        )
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
                    TableRef::LettersStudent,
                    param_user_id,
                    Some(previous),
                )
                .await;

                res
            }
            Err(_) => res,
        }
    }
}
