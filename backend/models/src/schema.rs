// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    company (id) {
        id -> Int4,
        name -> Text,
        address -> Text,
    }
}

diesel::table! {
    penarikan (id) {
        id -> Int4,
        user_id -> Int4,
        company_id -> Int4,
        end_date -> Date,
        verified -> Bool,
        verified_date -> Nullable<Date>,
        wave_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    penarikan_student (id) {
        id -> Int4,
        penarikan_id -> Int4,
        student_id -> Int4,
    }
}

diesel::table! {
    pengantaran (id) {
        id -> Int4,
        user_id -> Int4,
        company_id -> Int4,
        start_date -> Date,
        end_date -> Date,
        verified -> Bool,
        verified_date -> Nullable<Date>,
        wave_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    pengantaran_student (id) {
        id -> Int4,
        pengantaran_id -> Int4,
        student_id -> Int4,
    }
}

diesel::table! {
    permohonan (id) {
        id -> Int4,
        user_id -> Int4,
        company_id -> Int4,
        start_date -> Date,
        end_date -> Date,
        verified -> Bool,
        verified_date -> Nullable<Date>,
        wave_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    permohonan_student (id) {
        id -> Int4,
        permohonan_id -> Int4,
        student_id -> Int4,
    }
}

diesel::table! {
    session (id) {
        id -> Uuid,
        user_id -> Int4,
        invalidated -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        expires_at -> Timestamptz,
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
        class -> Text,
        #[max_length = 5]
        nis -> Varchar,
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
        start_date -> Date,
        end_date -> Nullable<Date>,
    }
}

diesel::joinable!(penarikan -> company (company_id));
diesel::joinable!(penarikan -> user (user_id));
diesel::joinable!(penarikan -> wave (wave_id));
diesel::joinable!(penarikan_student -> penarikan (penarikan_id));
diesel::joinable!(penarikan_student -> student (student_id));
diesel::joinable!(pengantaran -> company (company_id));
diesel::joinable!(pengantaran -> user (user_id));
diesel::joinable!(pengantaran -> wave (wave_id));
diesel::joinable!(pengantaran_student -> pengantaran (pengantaran_id));
diesel::joinable!(pengantaran_student -> student (student_id));
diesel::joinable!(permohonan -> company (company_id));
diesel::joinable!(permohonan -> user (user_id));
diesel::joinable!(permohonan -> wave (wave_id));
diesel::joinable!(permohonan_student -> permohonan (permohonan_id));
diesel::joinable!(permohonan_student -> student (student_id));
diesel::joinable!(session -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    company,
    penarikan,
    penarikan_student,
    pengantaran,
    pengantaran_student,
    permohonan,
    permohonan_student,
    session,
    signature,
    student,
    user,
    wave,
);
