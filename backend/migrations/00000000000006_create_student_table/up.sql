-- Your SQL goes here
CREATE TABLE
    "student" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        name TEXT NOT NULL,
        class TEXT NOT NULL,
        nis VARCHAR(5) NOT NULL
    );