CREATE TABLE users (
    username TEXT PRIMARY KEY NOT NULL COLLATE NOCASE,
    created_at INTEGER NOT NULL, -- unix ts

    -- private
    color_theme TEXT NOT NULL DEFAULT 'dark-theme'
) STRICT;

CREATE TABLE aliases (
    name TEXT PRIMARY KEY NOT NULL,
    content TEXT NOT NULL,
    author TEXT NOT NULL,
    created_at INTEGER NOT NULL, -- unix ts

    CONSTRAINT fk_author_assoc
        FOREIGN KEY (author)
        REFERENCES users (username)
) STRICT;
