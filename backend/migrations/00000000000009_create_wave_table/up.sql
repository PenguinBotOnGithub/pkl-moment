-- Your SQL goes here
CREATE TABLE
    "wave" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        start_year SMALLINT NOT NULL,
        end_year SMALLINT NOT NULL
    );
