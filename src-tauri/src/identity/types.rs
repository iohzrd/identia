use chrono::{offset::Utc, DateTime};
use ipfs_api::IpfsClient;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct AppState {
  pub db_pool: Pool<SqliteConnectionManager>,
  pub ipfs_client: IpfsClient,
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
  pub aux: Value,
  pub av: Value,
  pub dn: Value,
  pub following: Value,
  pub meta: Value,
  pub posts: Value,
  pub publisher: Value,
  pub ts: Value,
}

impl Identity {
  pub fn new(publisher: String) -> Identity {
    Identity {
      aux: json!([]),
      av: json!(""),
      dn: json!(""),
      following: json!([json!(publisher)]),
      meta: json!([]),
      posts: json!([]),
      publisher: json!(publisher),
      ts: json!(DateTime::timestamp_millis(&Utc::now())),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher {
  pub publisher: String,
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
