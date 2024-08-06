/* This file is generated and managed by dsync */
use crate::class::{Class, ClassJoined};
use crate::diesel::prelude::*;
use crate::log::Log;
use crate::schema::*;
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
#[diesel(table_name=student, primary_key(id), belongs_to(Class, foreign_key=class_id))]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub class_id: i32,
    pub nis: String,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct StudentJoined {
    pub id: i32,
    pub name: String,
    pub class: ClassJoined,
    pub nis: String,
    pub user: UserPublic,
}

#[derive(Debug, Serialize, Clone)]
pub struct StudentJoinedMini {
    pub id: i32,
    pub name: String,
    pub class: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=student)]
pub struct CreateStudent {
    pub name: String,
    pub class_id: i32,
    pub nis: String,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=student)]
pub struct UpdateStudent {
    pub name: Option<String>,
    pub class_id: Option<i32>,
    pub nis: Option<String>,
    pub user_id: Option<i32>,
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
    pub async fn create(
        db: &mut Connection,
        item: &CreateStudent,
        param_user_id: i32,
    ) -> QueryResult<Self> {
        use crate::schema::student::dsl::*;

        let res = diesel::insert_into(student)
            .values(item)
            .get_result::<Self>(db)
            .await;

        if let Ok(_) = res {
            Log::log(
                db,
                Operation::Create,
                TableRef::Student,
                param_user_id,
                None::<u8>,
            )
            .await;
        }

        res
    }

    pub async fn read(db: &mut Connection, param_id: i32) -> QueryResult<Option<StudentJoined>> {
        use crate::schema::class;
        use crate::schema::department;
        use crate::schema::student::dsl::*;
        use crate::schema::user;

        let res = student
            .filter(id.eq(param_id))
            .inner_join(class::table.inner_join(department::table))
            .inner_join(user::table)
            .select((
                id,
                name,
                class::id,
                class::grade,
                class::number,
                department::name,
                nis,
                user::all_columns,
            ))
            .first::<(i32, String, i32, i32, i32, String, String, User)>(db)
            .await
            .optional()?;
        let Some(mut res) = res else { return Ok(None) };

        Ok(Some(StudentJoined {
            id: res.0,
            name: mem::take(&mut res.1),
            class: ClassJoined {
                id: res.2,
                grade: res.3,
                number: res.4,
                department: mem::take(&mut res.5),
            },
            nis: mem::take(&mut res.6),
            user: res.7.public(),
        }))
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<StudentJoined>> {
        use crate::schema::class;
        use crate::schema::department;
        use crate::schema::student::dsl::*;
        use crate::schema::user;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = student.count().get_result(db).await?;
        let items = student
            .limit(page_size)
            .inner_join(class::table.inner_join(department::table))
            .inner_join(user::table)
            .select((
                id,
                name,
                class::id,
                class::grade,
                class::number,
                department::name,
                nis,
                user::all_columns,
            ))
            .offset(page * page_size)
            .load::<(i32, String, i32, i32, i32, String, String, User)>(db)
            .await?
            .iter_mut()
            .map(
                |(s_id, s_name, c_id, grade, num, d_name, s_nis, user)| StudentJoined {
                    id: *s_id,
                    name: mem::take(s_name),
                    class: ClassJoined {
                        id: *c_id,
                        grade: *grade,
                        number: *num,
                        department: mem::take(d_name),
                    },
                    nis: mem::take(s_nis),
                    user: user.public(),
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
        item: &UpdateStudent,
        param_user_id: i32,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::student::dsl::*;

        let previous = Student::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(None);
        };

        let res = diesel::update(student.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
            .await
            .optional();

        if let Ok(Some(_)) = res {
            Log::log(
                db,
                Operation::Update,
                TableRef::Student,
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
        use crate::schema::student::dsl::*;

        let previous = Student::read(db, param_id).await?;
        let Some(previous) = previous else {
            return Ok(0);
        };

        let res = diesel::delete(student.filter(id.eq(param_id)))
            .execute(db)
            .await;

        match res {
            Ok(n) => {
                if n <= 0 {
                    return res;
                }

                Log::log(
                    db,
                    Operation::Delete,
                    TableRef::Student,
                    param_user_id,
                    Some(previous),
                )
                .await;

                res
            }
            Err(_) => res,
        }
    }

    pub async fn filter_by_name(
        db: &mut Connection,
        param_name: String,
    ) -> QueryResult<Vec<StudentJoinedMini>> {
        use crate::schema::class;
        use crate::schema::department;
        use crate::schema::student::dsl::*;

        Ok(student
            .filter(name.ilike(param_name))
            .inner_join(class::table.inner_join(department::table))
            .select((id, name, class::number, class::grade, department::name))
            .load::<(i32, String, i32, i32, String)>(db)
            .await?
            .into_iter()
            .map(|(s_id, s_name, c_num, c_grade, d_name)| StudentJoinedMini {
                id: s_id,
                name: s_name,
                class: format!("{c_grade} {d_name}-{c_num}"),
            })
            .collect())
    }
}
