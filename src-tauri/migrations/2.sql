CREATE TABLE "comments" (
    "type"              TEXT,
    "from"              TEXT,
    "topic"             TEXT,
    "data"              BLOB,
    "sequenceNumber"    BIGINT,
    "signature"         BLOB,
    "key"               BLOB,
    "timestamp"         INTEGER,
    "inReplyTo"         TEXT,
    PRIMARY KEY("sequenceNumber")
);
