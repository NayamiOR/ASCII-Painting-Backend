-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS paintings (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    author_id SERIAL NOT NULL REFERENCES users (id),
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    favorite_num INT NOT NULL DEFAULT 0,
    like_num INT NOT NULL DEFAULT 0,
    state INT NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS avatars (
    id SERIAL PRIMARY KEY,
    user_id SERIAL NOT NULL REFERENCES users (id),
    file_path VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE type action_type AS ENUM('like', 'favorite');
CREATE TABLE IF NOT EXISTS user_actions (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users (id),      -- 关联用户表
    painting_id BIGINT NOT NULL REFERENCES paintings (id), -- 关联画作表
    action_type action_type NOT NULL,  -- 操作类型，枚举值（like, favorite）
    action_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 操作时间
    UNIQUE (user_id, painting_id, action_type)  -- 唯一约束，防止重复点赞或收藏
);
