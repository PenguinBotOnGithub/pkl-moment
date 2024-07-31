-- Your SQL goes here
CREATE TABLE "journal" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        student_id INT NOT NULL,
        company_id INT NOT NULL,
        division VARCHAR NOT NULL,
        entry_date DATE NOT NULL,
        start_time TIME NOT NULL,
        end_time TIME NOT NULL,
        activity VARCHAR NOT NULL,
        img_url VARCHAR NOT NULL,
        extra VARCHAR NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        CONSTRAINT fk_student FOREIGN KEY ("student_id") REFERENCES "student" ("id"),
        CONSTRAINT fk_company FOREIGN KEY ("company_id") REFERENCES "company" ("id"),
        UNIQUE(student_id, entry_date)
);