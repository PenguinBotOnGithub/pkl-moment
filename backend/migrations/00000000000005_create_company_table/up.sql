-- Your SQL goes here
CREATE TABLE
    "company" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        name TEXT NOT NULL,
        address TEXT NOT NULL,
        mou_url TEXT
    );
