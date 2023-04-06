CREATE TABLE users (
    username TEXT PRIMARY KEY NOT NULL COLLATE NOCASE,
    created_at INTEGER NOT NULL, -- unix ts

    -- private
    color_theme TEXT NOT NULL DEFAULT 'dark-theme'
) STRICT;

CREATE TABLE alias_types (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE
) STRICT;

CREATE TABLE aliases (
    name TEXT PRIMARY KEY NOT NULL,
    content TEXT NOT NULL,
    "type" INTEGER NOT NULL,
    author TEXT NOT NULL,
    created_at INTEGER NOT NULL, -- unix ts

    CONSTRAINT fk_author_assoc
        FOREIGN KEY (author)
        REFERENCES users (username),

    CONSTRAINT fk_type_assoc
        FOREIGN KEY ("type")
        REFERENCES alias_types (id)
) STRICT;

INSERT INTO alias_types (id, name)
VALUES
(1, "text"),
(2, "image"),
(3, "gif"),
(4, "emote"),
(5, "animatedEmote");
