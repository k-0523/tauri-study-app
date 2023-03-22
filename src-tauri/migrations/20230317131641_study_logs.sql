-- Add migration script here
CREATE TABLE study_logs (
  id INTEGER NOT NULL PRIMARY KEY,
  content TEXT NOT NULL,
  date NUMERIC,
  created_at TIMESTAMP,
  updated_at TIMESTAMP
)