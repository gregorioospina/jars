-- Your SQL goes here
CREATE TABLE jars (
  id SERIAL PRIMARY KEY,
  nickname VARCHAR NOT NULL,
  place TEXT NOT NULL,
  queries INT NOT NULL DEFAULT 0,
  bar TEXT NOT NULL
)