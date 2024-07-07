-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    player_id SERIAL UNIQUE,
    student_id VARCHAR(16) UNIQUE NOT NULL,
    username VARCHAR(32) UNIQUE NOT NULL,
    password VARCHAR(64) NOT NULL
    -- entered_room_id BIGINT NULL DEFAULT NULL
);
