-- Add migration script here
CREATE TABLE IF NOT EXISTS movies (
    id SERIAL PRIMARY KEY,
    title varchar(255) NOT NULL,
    short_description text NOT NULL,
    embeddings vector(1024)
);