-- Your SQL goes here

CREATE TABLE customers (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

INSERT INTO customers ("name") values ('Landeed'), ('Test'), ('Empty');