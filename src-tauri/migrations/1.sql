CREATE TABLE "identities" (
    "cid"           TEXT NOT NULL,
    "avatar"        TEXT NOT NULL,
    "description"   TEXT NOT NULL,
    "display_name"  TEXT NOT NULL,
    "following"     BLOB NOT NULL,
    "meta"          BLOB NOT NULL,
    "posts"         BLOB NOT NULL,
    "publisher"     TEXT NOT NULL,
    "timestamp"     INTEGER NOT NULL,
    PRIMARY KEY("publisher")
);

CREATE TABLE "posts" (
    "cid"         TEXT NOT NULL,
    "body"        TEXT NOT NULL,
    "files"       BLOB NOT NULL,
    "meta"        BLOB NOT NULL,
    "publisher"   TEXT NOT NULL,
    "timestamp"   INTEGER NOT NULL,
    FOREIGN KEY("publisher") REFERENCES "identities"("publisher"),
    PRIMARY KEY("cid")
);
