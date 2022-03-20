CREATE TABLE identities (
    avatar        TEXT,
    cid           TEXT,
    description   TEXT,
    display_name  TEXT,
    following     BLOB,
    meta          BLOB,
    posts         BLOB,
    publisher     TEXT primary key,
    timestamp     INT
  );

CREATE TABLE posts (
    cid         TEXT,
    body        TEXT,
    files       BLOB,
    meta        BLOB,
    publisher   TEXT,
    timestamp   INT primary key
  );
  