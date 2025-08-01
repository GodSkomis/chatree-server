

CREATE TABLE users (
    id BIGINT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    hashed_password TEXT NOT NULL,

    status TEXT DEFAULT NULL,
    bio TEXT DEFAULT NULL,

    is_banned BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE
);