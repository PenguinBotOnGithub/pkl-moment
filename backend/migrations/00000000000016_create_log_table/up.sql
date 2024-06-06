-- Your SQL goes here
CREATE TABLE "log" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        operation_type operation NOT NULL,
        table_affected table_ref NOT NULL,
        user_id INT NOT NULL,
        logged_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        CONSTRAINT fk_user FOREIGN KEY ("user_id") REFERENCES "user" ("id")
);