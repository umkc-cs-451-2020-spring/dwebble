-- Your SQL goes here


CREATE TYPE permission_enum AS ENUM ('admin', 'standard');

CREATE TABLE user_ (
       id SERIAL PRIMARY KEY,
       username VARCHAR (10) NOT NULL UNIQUE,
       f_name VARCHAR(100) NOT NULL,
       l_name VARCHAR(100) NOT NULL,
       email VARCHAR(100) NOT NULL UNIQUE,
       pw_hash TEXT NOT NULL,
       user_auth permission_enum NOT NULL DEFAULT 'standard'
);
