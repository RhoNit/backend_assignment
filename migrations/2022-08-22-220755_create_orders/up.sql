-- Your SQL goes here

CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  customer_id INT NOT NULL REFERENCES customers,
  created_at INT NOT NULL,
  status TEXT NOT NULL,
  premium BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO orders ("customer_id", "created_at", "status", "premium") values 
  (1, 1609491661, 'Created', FALSE),
  (1, 1609491661, 'Completed', TRUE),
  (1, 1672563661, 'Paid', FALSE),
  (2, 1672563661, 'Created', TRUE);