-- Your SQL goes here

CREATE TABLE payments (
  id SERIAL PRIMARY KEY,
  order_id INT NOT NULL REFERENCES orders,
  created_at INT NOT NULL,
  status TEXT NOT NULL
);

INSERT INTO payments ("order_id", "created_at", "status") values 
  (2, 1609491661, 'Completed'),
  (2, 1672563661, 'Refunded'),
  (3, 1672563661, 'Completed'),
  (4, 1672563661, 'Refunded');
