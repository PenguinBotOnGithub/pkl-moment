use diesel::sql_types::Bool;
use diesel::BoxableExpression;
use diesel_async::AsyncPgConnection;

pub use diesel;

pub mod class;
pub mod company;
pub mod department;
pub mod invalidated_jwt;
pub mod journal;
pub mod letters;
pub mod letters_student;
pub mod log;
pub mod schema;
pub mod signature;
pub mod student;
pub mod tenure;
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
