use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
  pub body: String,
  pub cid: String,
  pub from: String,
  pub kind: String,
  pub in_response_to: String,
  pub topic: String,
  pub ts: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuxObj {
  pub key: String,
  pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
  pub aux: Value,
  pub av: String,
  pub dn: String,
  pub following: Value,
  pub meta: Value,
  pub posts: Value,
  pub publisher: String,
  pub ts: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
  pub aux: Value,
  pub body: String,
  pub files: Value,
  pub filesRoot: Option<String>,
  pub files_root: Option<String>,
  pub meta: Value,
  pub publisher: String,
  pub ts: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
  pub cid: String,
  pub post: Post,
}
