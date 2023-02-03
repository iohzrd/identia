CREATE TABLE "topic_post" (
    -- pubsub
    "type"              TEXT NOT NULL,
    "from"              TEXT NOT NULL,
    "topic"             TEXT NOT NULL,
    "data"              BLOB NOT NULL,
    "sequenceNumber"    INTEGER NOT NULL,
    "signature"         BLOB NOT NULL,
    "key"               BLOB NOT NULL,
    -- custom
    "timestamp"         INTEGER NOT NULL
    "inReplyTo"         TEXT NOT NULL,
    "body"              TEXT NOT NULL,
    "files"             BLOB NOT NULL,
);
