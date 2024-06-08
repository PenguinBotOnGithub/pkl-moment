use diesel::sql_types::Bool;
use diesel::BoxableExpression;
use diesel_async::AsyncPgConnection;

pub mod company;
pub mod invalidated_jwt;
pub mod log;
pub mod penarikan;
pub mod penarikan_student;
pub mod pengantaran;
pub mod pengantaran_student;
pub mod permohonan;
pub mod permohonan_student;
pub mod schema;
pub mod signature;
pub mod student;
pub mod types;
pub mod user;
pub mod wave;

pub struct LetterFilter {
    company_id: Option<i32>,
    student_id: Option<i32>,
    wave_id: Option<i32>,
}

pub type BoxedExpression<'a, T> =
    Box<dyn BoxableExpression<T, AsyncPgConnection, SqlType = Bool> + 'a>;
