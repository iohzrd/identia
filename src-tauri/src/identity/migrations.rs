pub const CREATE_IDENTITIES_TABLE: &str = "
    create table if not exists identities (
    av          text,
    dn          text,
    following   text,
    meta        text,
    posts       text,
    publisher   text primary key,
    ts          int
  )";

pub const CREATE_POSTS_TABLE: &str = "
    create table if not exists posts (
    cid         text,
    body        text,
    files       text,
    meta        text,
    publisher   text,
    ts          int primary key
  )";
