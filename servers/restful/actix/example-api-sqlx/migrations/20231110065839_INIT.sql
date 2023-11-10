-- Add migration script here
CREATE TABLE IF NOT EXISTS persons (
    id BIGSERIAL NOT NULL,
    name varchar(256) NOT NULL,
    surname varchar(256) NOT NULL,
    age int NOT NULL,
    address varchar(512),
    tel varchar(32),
    PRIMARY KEY (id)
);
