-- Your SQL goes here
CREATE TABLE "tenure" (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    student_id INT NOT NULL,
    advsch_id INT NULL,
    advdudi_id INT NULL,
    letters_id INT NOT NULL,
    CONSTRAINT fk_student FOREIGN KEY ("student_id") REFERENCES "student" ("id"),
    CONSTRAINT fk_advsch FOREIGN KEY ("advsch_id") REFERENCES "user" ("id"),
    CONSTRAINT fk_advdudi FOREIGN KEY ("advdudi_id") REFERENCES "user" ("id"),
    CONSTRAINT fk_letters FOREIGN KEY ("letters_id") REFERENCES "letters" ("id")
);