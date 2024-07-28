-- Your SQL goes here
CREATE TABLE
    "student" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        name TEXT NOT NULL,
        class_id INT NOT NULL,
        nis VARCHAR(5) NOT NULL,
        CONSTRAINT fk_class FOREIGN KEY ("class_id") REFERENCES "class" ("id")
    );
