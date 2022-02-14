CREATE TABLE handle (
    id integer primary key,
    passhash varchar not null,
    `name` varchar
);

CREATE TABLE board (
    id integer primary key,
    code varchar
);

CREATE TABLE post (
    id integer PRIMARY key,
    handle integer references handle(id) not null,
    board integer references board(id) not null,
    `subject` text,
    body text
);
