-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(255) NOT NULL,
    email      VARCHAR(255) NOT NULL UNIQUE,
    password   VARCHAR(255) NOT NULL,
    created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS paintings
(
    id           SERIAL PRIMARY KEY,
    name         VARCHAR(255) NOT NULL,
    author_id    SERIAL       NOT NULL REFERENCES users (id),
    content      TEXT         NOT NULL,
    created_at   TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    favorite_num INT          NOT NULL DEFAULT 0,
    like_num     INT          NOT NULL DEFAULT 0,
    state        INT          NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS avatars
(
    id         SERIAL PRIMARY KEY,
    user_id    SERIAL       NOT NULL REFERENCES users (id),
    file_path  VARCHAR(255) NOT NULL,
    created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP
);