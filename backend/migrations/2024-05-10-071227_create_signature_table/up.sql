-- Your SQL goes here
CREATE TABLE
    "signature" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        name TEXT NOT NULL,
        title TEXT NOT NULL,
        image TEXT NOT NULL
    );