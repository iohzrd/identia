CREATE TABLE "topics" (
    "topic" TEXT NOT NULL UNIQUE,
    PRIMARY KEY("topic")
);

CREATE TABLE "topic_post" (
    "type"              TEXT NOT NULL,
    "from"              TEXT NOT NULL,
    "topic"             TEXT NOT NULL,
    "data"              BLOB NOT NULL,
    "sequenceNumber"    INTEGER NOT NULL,
    "signature"         BLOB NOT NULL,
    "key"               BLOB NOT NULL,
    "timestamp"         INTEGER NOT NULL,
    PRIMARY KEY("sequenceNumber")
);
