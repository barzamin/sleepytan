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

CREATE TABLE thread (
    id INTEGER PRIMARY KEY,
    board INTEGER REFERENCES board(id) NOT NULL,
    subject TEXT NOT NULL,
    bump_ts TIMESTAMP NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE post (
    id INTEGER PRIMARY KEY,
    thread INTEGER REFERENCES thread(id) NOT NULL,
    handle BLOB REFERENCES handle(id) NOT NULL,
    attachment INTEGER REFERENCES attachment(id),

    body TEXT NOT NULL,
    create_ts TIMESTAMP NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE attachment (
    id INTEGER PRIMARY KEY,
    `name` VARCHAR NOT NULL,
    `blobhash` BLOB NOT NULL
);

-- triggers --
CREATE TRIGGER bump_thread AFTER INSERT ON post
BEGIN
    UPDATE thread SET bump_ts = NEW.create_ts
    WHERE id = NEW.thread;
    -- if saging were implemented, it'd be done here.
END;