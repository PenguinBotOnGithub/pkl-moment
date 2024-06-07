-- Your SQL goes here
CREATE TYPE table_ref AS ENUM (
    'user',
    'wave',
    'company',
    'student',
    'signature',
    'permohonan',
    'permohonan_student',
    'pengantaran',
    'pengantaran_student',
    'penarikan',
    'penarikan_student'
);