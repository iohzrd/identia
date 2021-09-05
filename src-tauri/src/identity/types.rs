use chrono::{offset::Utc, DateTime};
use ipfs_api::IpfsClient;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};

pub struct AppState {
  pub db_pool: Pool<SqliteConnectionManager>,
  pub ipfs_client: IpfsClient,
  pub ipfs_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddObj {
  pub path: String,
  pub content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuxObj {
  pub key: String,
  pub value: String,
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
pub struct Feed {
  pub feed: Vec<PostResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
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

impl Identity {
  pub fn new(publisher: String, ts: i64) -> Identity {
    Identity {
      aux: Vec::new(),
      av: String::from(""),
      dn: String::from(""),
      following: Vec::new(),
      meta: Vec::new(),
      posts: Vec::new(),
      publisher: String::from(publisher),
      ts: ts,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
  pub aux: Vec<AuxObj>,
  pub body: String,
  pub files: Vec<String>,
  pub meta: Vec<String>,
  pub publisher: String,
  pub ts: i64,
}
impl Post {
  pub fn new(
    aux: Vec<AuxObj>,
    body: String,
    files: Vec<String>,
    meta: Vec<String>,
    publisher: String,
  ) -> Post {
    Post {
      aux: aux,
      body: body,
      files: files,
      meta: meta,
      publisher: publisher,
      ts: DateTime::timestamp_millis(&Utc::now()),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRequest {
  pub aux: Vec<AuxObj>,
  pub body: String,
  pub files: Vec<String>,
  pub meta: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
  pub cid: String,
  pub post: Post,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher {
  pub publisher: String,
}
