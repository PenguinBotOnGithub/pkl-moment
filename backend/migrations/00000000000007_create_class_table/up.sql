-- Your SQL goes here
CREATE TABLE "class" (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    grade INT NOT NULL,
    number INT NOT NULL,
    department_id INT NOT NULL,
    CONSTRAINT fk_department FOREIGN KEY ("department_id") REFERENCES "department" ("id"),
    UNIQUE("number", "department_id")
);
