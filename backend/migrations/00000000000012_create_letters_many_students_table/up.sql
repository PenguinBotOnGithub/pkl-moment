-- Your SQL goes here
CREATE TABLE
    "permohonan_student" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        permohonan_id INT NOT NULL,
        student_id INT NOT NULL,
        CONSTRAINT fk_permohonan FOREIGN KEY ("permohonan_id") REFERENCES "permohonan" ("id") ON DELETE CASCADE,
        CONSTRAINT fk_student FOREIGN KEY ("student_id") REFERENCES "student" ("id") ON DELETE CASCADE
    );

CREATE TABLE
    "pengantaran_student" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        pengantaran_id INT NOT NULL,
        student_id INT NOT NULL,
        CONSTRAINT fk_pengantaran FOREIGN KEY ("pengantaran_id") REFERENCES "pengantaran" ("id") ON DELETE CASCADE,
        CONSTRAINT fk_student FOREIGN KEY ("student_id") REFERENCES "student" ("id") ON DELETE CASCADE
    );

CREATE TABLE
    "penarikan_student" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        penarikan_id INT NOT NULL,
        student_id INT NOT NULL,
        CONSTRAINT fk_penarikan FOREIGN KEY ("penarikan_id") REFERENCES "penarikan" ("id") ON DELETE CASCADE,
        CONSTRAINT fk_student FOREIGN KEY ("student_id") REFERENCES "student" ("id") ON DELETE CASCADE
    );