use chrono::{offset::Utc, DateTime};
use ipfs_api::IpfsClient;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct AppState {
  pub db_pool: Pool<SqliteConnectionManager>,
  pub ipfs_client: IpfsClient,
  pub ipfs_id: String,
}

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
  // pub aux: Value,
  pub aux: Vec<AuxObj>,
  pub av: String,
  pub dn: String,
  // pub following: Value,
  pub following: Vec<String>,
  // pub meta: Value,
  pub meta: Vec<String>,
  // pub posts: Value,
  pub posts: Vec<String>,
  pub publisher: String,
  pub ts: i64,
}

impl Identity {
  pub fn new(publisher: String) -> Identity {
    Identity {
      // aux: json!([]),
      aux: vec![],
      av: String::from(""),
      dn: String::from(""),
      // following: json!([json!(publisher)]),
      following: vec![publisher.clone()],
      // meta: json!([]),
      meta: vec![],
      // posts: json!([]),
      posts: vec![],
      publisher: String::from(publisher),
      ts: DateTime::timestamp_millis(&Utc::now()),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher {
  pub publisher: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
  pub aux: Vec<AuxObj>,
  pub body: String,
  pub files: Vec<String>,
  pub files_cid: Option<String>,
  pub meta: Vec<String>,
  pub publisher: String,
  pub ts: i64,
}
impl Post {
  pub fn new() -> Post {
    Post {
      aux: vec![],
      body: String::from(""),
      // files: vec![publisher.clone()],
      files: vec![],
      files_cid: None,
      meta: vec![],
      // publisher: String::from(publisher),
      publisher: String::from(""),
      ts: DateTime::timestamp_nanos(&Utc::now()),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRequest {
  pub body: String,
  pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
  pub cid: String,
  pub post: Post,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed {
  pub feed: Vec<PostResponse>,
}
