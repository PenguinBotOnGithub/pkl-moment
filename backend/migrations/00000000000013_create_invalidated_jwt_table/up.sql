-- Your SQL goes here
CREATE TABLE
    "invalidated_jwt" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        jwt TEXT NOT NULL,
        invalidated_timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        expires_at TIMESTAMPTZ NOT NULL
    );