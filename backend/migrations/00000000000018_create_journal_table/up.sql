-- Your SQL goes here
CREATE TABLE "journal" (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    tenure_id INT NOT NULL,
    division VARCHAR NOT NULL,
    entry_date DATE NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    activity VARCHAR NOT NULL,
    img_url VARCHAR NOT NULL,
    extra VARCHAR NULL,
    verified_sch BOOL NOT NULL DEFAULT FALSE,
    verified_dudi BOOL NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_tenure FOREIGN KEY ("tenure_id") REFERENCES "tenure" ("id"),
    UNIQUE(tenure_id, entry_date)
);

CREATE TRIGGER updated_at_trigger BEFORE
UPDATE ON "journal" FOR EACH ROW EXECUTE FUNCTION change_updated_at_row ();
