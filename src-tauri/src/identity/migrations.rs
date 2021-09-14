pub const CREATE_IDENTITIES_TABLE: &str = "
    create table if not exists identities (
    avatar        text,
    cid           text,
    description   text,
    display_name  text,
    following     text,
    meta          text,
    posts         text,
    publisher     text primary key,
    timestamp     int
  )";

pub const CREATE_POSTS_TABLE: &str = "
    create table if not exists posts (
    cid         text,
    body        text,
    files       text,
    meta        text,
    publisher   text,
    timestamp   int primary key
  )";
