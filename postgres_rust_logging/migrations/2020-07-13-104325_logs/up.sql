-- Your SQL goes here
CREATE TABLE logs (
  log_id INT PRIMARY KEY,
  status_code INT,
  message TEXT,
  time TIMESTAMP,
  resolved BOOLEAN DEFAULT false
);