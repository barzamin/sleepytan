CREATE TABLE handle (
    id BLOB PRIMARY KEY,
    passhash VARCHAR NOT NULL,
    `name` VARCHAR NOT NULL,
    `desc` TEXT,
    create_ts TIMESTAMP NOT NULL DEFAULT(datetime('now'))
);

CREATE TABLE board (
    id INTEGER PRIMARY KEY,
    code VARCHAR NOT NULL UNIQUE,
    `desc` TEXT NOT NULL
);

CREATE TABLE post (
    id INTEGER PRIMARY KEY,
    board INTEGER REFERENCES board(id) NOT NULL,
    handle blob REFERENCES handle(id) NOT NULL,
    parent INTEGER REFERENCES post(id) ON DELETE CASCADE, -- if null: we're root of a thread
    attachment INTEGER REFERENCES attachment(id),

    subject TEXT NOT NULL,
    body TEXT NOT NULL,
    create_ts TIMESTAMP NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE attachment (
    id INTEGER PRIMARY KEY,
    `name` VARCHAR NOT NULL,
    `blobhash` BLOB NOT NULL
);

CREATE VIEW threads AS
SELECT
    post.id,
    post.board,
    post.handle,
    post.subject,
    post.body,
    post.create_ts
FROM post
WHERE
    post.parent IS NULL;
