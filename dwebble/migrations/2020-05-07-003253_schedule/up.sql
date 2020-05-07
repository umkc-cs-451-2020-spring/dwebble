-- Your SQL goes here

CREATE TYPE semester_enum AS ENUM ('summer', 'fall', 'winter', 'spring');
-- CREATE DOMAIN year SMALLINT CHECK (VALUE > 2000);

CREATE TABLE schedule (
       schedule_id SERIAL PRIMARY KEY,
       instructor_id INT,
       schedule_data JSONB NOT NULL,
       semester semester_enum NOT NULL,
       -- yr year NOT NULL,
       year SMALLINT CHECK (year > 2000),
       CONSTRAINT fk_schedule_instructor_id
                  FOREIGN KEY (instructor_id)
                  REFERENCES instructor (instructor_id)
                  ON DELETE SET NULL
);
