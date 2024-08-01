// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "operation"))]
    pub struct Operation;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "table_ref"))]
    pub struct TableRef;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    class (id) {
        id -> Int4,
        grade -> Int4,
        number -> Int4,
        department_id -> Int4,
    }
}

diesel::table! {
    company (id) {
        id -> Int4,
        name -> Text,
        address -> Text,
        mou_url -> Nullable<Text>,
    }
}

diesel::table! {
    department (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    invalidated_jwt (id) {
        id -> Int4,
        jwt -> Text,
        invalidated_timestamp -> Timestamptz,
        expires_at -> Timestamptz,
    }
}

diesel::table! {
    journal (id) {
        id -> Int4,
        student_id -> Int4,
        company_id -> Int4,
        division -> Varchar,
        entry_date -> Date,
        start_time -> Time,
        end_time -> Time,
        activity -> Varchar,
        img_url -> Varchar,
        extra -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    letters (id) {
        id -> Int4,
        user_id -> Int4,
        company_id -> Int4,
        start_date -> Date,
        end_date -> Date,
        verified -> Bool,
        verified_at -> Nullable<Timestamptz>,
        wave_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    letters_student (id) {
        id -> Int4,
        letters_id -> Int4,
        student_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Operation;
    use super::sql_types::TableRef;

    log (id) {
        id -> Int4,
        operation_type -> Operation,
        table_affected -> TableRef,
        user_id -> Int4,
        snapshot -> Nullable<Text>,
        logged_at -> Timestamptz,
    }
}

diesel::table! {
    signature (id) {
        id -> Int4,
        name -> Text,
        title -> Text,
    }
}

diesel::table! {
    student (id) {
        id -> Int4,
        name -> Text,
        class_id -> Int4,
        #[max_length = 5]
        nis -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    tenure (id) {
        id -> Int4,
        student_id -> Int4,
        advsch_id -> Int4,
        advdudi_id -> Int4,
        letters_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    user (id) {
        id -> Int4,
        #[max_length = 20]
        username -> Varchar,
        password -> Text,
        role -> UserRole,
    }
}

diesel::table! {
    wave (id) {
        id -> Int4,
        start_year -> Int2,
        end_year -> Int2,
    }
}

diesel::joinable!(class -> department (department_id));
diesel::joinable!(journal -> company (company_id));
diesel::joinable!(journal -> student (student_id));
diesel::joinable!(letters -> company (company_id));
diesel::joinable!(letters -> user (user_id));
diesel::joinable!(letters -> wave (wave_id));
diesel::joinable!(letters_student -> letters (letters_id));
diesel::joinable!(letters_student -> student (student_id));
diesel::joinable!(log -> user (user_id));
diesel::joinable!(student -> class (class_id));
diesel::joinable!(student -> user (user_id));
diesel::joinable!(tenure -> letters (letters_id));
diesel::joinable!(tenure -> student (student_id));

diesel::allow_tables_to_appear_in_same_query!(
    class,
    company,
    department,
    invalidated_jwt,
    journal,
    letters,
    letters_student,
    log,
    signature,
    student,
    tenure,
    user,
    wave,
);
