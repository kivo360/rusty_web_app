-- Your SQL goes here
CREATE TABLE events
(
    id SERIAL PRIMARY KEY,
    streamer_name VARCHAR(255) NOT NULL,
    event_type VARCHAR(255) NOT NULL,
    viewer_name TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)