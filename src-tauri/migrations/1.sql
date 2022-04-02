CREATE TABLE identities (
    avatar        TEXT,
    cid           TEXT,
    description   TEXT,
    display_name  TEXT,
    following     BLOB,
    meta          BLOB,
    posts         BLOB,
    publisher     TEXT PRIMARY KEY,
    timestamp     INT
  );

CREATE TABLE posts (
    cid         TEXT,
    body        TEXT,
    files       BLOB,
    meta        BLOB,
    publisher   TEXT,
    timestamp   INT PRIMARY KEY
  );
