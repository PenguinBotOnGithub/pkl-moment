-- Your SQL goes here
CREATE TABLE
    "wave" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        start_date DATE NOT NULL,
        end_date DATE
    );