-- Your SQL goes here
CREATE TABLE
    "session" (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid (),
        user_id INT NOT NULL,
        invalidated BOOLEAN NOT NULL DEFAULT FALSE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        expires_at TIMESTAMPTZ NOT NULL DEFAULT (CURRENT_TIMESTAMP + INTERVAL '7 days'),
        CONSTRAINT fk_user FOREIGN KEY ("user_id") REFERENCES "user" ("id")
    );

CREATE TRIGGER updated_at_trigger BEFORE
UPDATE ON "session" FOR EACH ROW EXECUTE FUNCTION change_updated_at_row ();