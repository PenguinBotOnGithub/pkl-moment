-- Your SQL goes here
CREATE TABLE
    "session" (
        id UUID DEFAULT gen_random_uuid (),
        user_id INT NOT NULL,
        invalidated BOOLEAN NOT NULL DEFAULT FALSE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        expires_at TIMESTAMPTZ NOT NULL DEFAULT (CURRENT_TIMESTAMP + 7),
        CONSTRAINT fk_user FOREIGN KEY ("user_id") REFERENCES "user" ("id")
    );