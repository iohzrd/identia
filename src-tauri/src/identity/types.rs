use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Comment {
  pub body: String,
  pub cid: String,
  pub from: String,
  pub kind: String,
  pub in_response_to: String,
  pub topic: String,
  pub ts: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AuxObj {
  pub key: String,
  pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Identity {
  pub aux: Vec<AuxObj>,
  pub av: String,
  pub dn: String,
  pub following: Vec<String>,
  pub meta: Vec<String>,
  pub posts: Vec<String>,
  pub publisher: String,
  pub ts: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Post {
  pub aux: Vec<String>,
  pub body: String,
  pub files: Vec<String>,
  pub filesRoot: Option<String>,
  pub files_root: Option<String>,
  pub meta: Vec<String>,
  pub publisher: String,
  pub ts: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PostResponse {
  pub cid: String,
  pub post: Post,
}
