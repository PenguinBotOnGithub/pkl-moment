-- Your SQL goes here
CREATE TABLE
    "letters_student" (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        letters_id INT NOT NULL,
        student_id INT NOT NULL,
        CONSTRAINT fk_letters FOREIGN KEY ("letters_id") REFERENCES "letters" ("id") ON DELETE CASCADE,
        CONSTRAINT fk_student FOREIGN KEY ("student_id") REFERENCES "student" ("id") ON DELETE CASCADE
);