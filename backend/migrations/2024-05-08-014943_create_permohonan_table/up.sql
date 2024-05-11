-- Your SQL goes here
CREATE TABLE
    "permohonan" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        user_id INT NOT NULL,
        company_id INT NOT NULL,
        start_date DATE NOT NULL,
        end_date DATE NOT NULL,
        verified BOOLEAN NOT NULL DEFAULT FALSE,
        verified_date DATE,
        wave_id INT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL,
        updated_at TIMESTAMPTZ NOT NULL,
        CONSTRAINT fk_user FOREIGN KEY ("user_id") REFERENCES "user" ("id"),
        CONSTRAINT fk_company FOREIGN KEY ("company_id") REFERENCES "company" ("id"),
        CONSTRAINT fk_wave FOREIGN KEY ("wave_id") REFERENCES "wave" ("id")
    );