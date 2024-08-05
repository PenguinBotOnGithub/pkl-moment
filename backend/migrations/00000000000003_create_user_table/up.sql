-- Your SQL goes here
CREATE TABLE
    "user" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        username VARCHAR(50) NOT NULL UNIQUE,
        password TEXT NOT NULL,
        role user_role NOT NULL
    );