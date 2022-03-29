CREATE TABLE handle (
    id blob primary key,
    passhash varchar not null,
    `name` varchar not null,
    `desc` text,
    create_ts timestamp not null default (datetime('now'))
);

CREATE TABLE board (
    id integer primary key,
    code varchar not null,
    `desc` text not null
);

CREATE TABLE post (
    id integer primary key,
    handle blob references handle(id) not null,
    board integer references board(id) not null,
    `subject` text not null,
    body text not null,
    create_ts timestamp not null default (datetime('now'))
);
