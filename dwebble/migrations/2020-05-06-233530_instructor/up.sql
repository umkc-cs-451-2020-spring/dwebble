-- Your SQL goes here
-- PTAdjunct -> Part-time Adjunct
-- FTAdjunct -> Full-time Adjunct
-- GraduateTA -> Graduate Teaching Assistant (PhD or Masters)

CREATE TYPE instructor_enum AS ENUM ('tenured', 'tenured_track', 'non_tenured',
                                     'pt_adjunct', 'ft_adjunct', 'graduate_ta');

CREATE TABLE instructor (
       instructor_id SERIAL PRIMARY KEY,
       type_ instructor_enum NOT NULL,
       user_id INT DEFAULT NULL,
       f_name VARCHAR(100) NOT NULL,
       l_name VARCHAR(100) NOT NULL,
       CONSTRAINT fk_instructor_user_id
                  FOREIGN KEY (user_id)
                  REFERENCES user_ (id)
                  ON DELETE SET NULL
);
