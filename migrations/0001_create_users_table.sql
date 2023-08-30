CREATE TABLE IF NOT EXISTS users (
    username VARCHAR(50) NOT NULL,
    email    VARCHAR(255) NOT NULL,
    passw    VARCHAR(255) NOT NULL,
    UNIQUE(username),
    PRIMARY KEY(username)
)
