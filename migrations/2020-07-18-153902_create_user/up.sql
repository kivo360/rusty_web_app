-- Your SQL goes here
CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    api_key TEXT NOT NULL,
    favorite_streamer VARCHAR(255) NOT NULL
)