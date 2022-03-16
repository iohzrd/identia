// Copyright 2021-2022, iohzrd <iohzrd@protonmail.com>
// SPDX-License-Identifier: AGPL-3.0

use chrono::{offset::Utc, DateTime};
use ipfs_api::IpfsClient;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{json, Value};

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
  pub avatar: String,
  pub description: String,
  pub display_name: String,
  pub following: Vec<String>,
  pub meta: Value,
  pub posts: Vec<String>,
  pub publisher: String,
  pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityRequest {
  pub publisher: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityResponse {
  pub cid: String,
  pub identity: Identity,
}

impl Identity {
  pub fn new(publisher: String, timestamp: i64) -> Identity {
    Identity {
      avatar: String::from(""),
      description: String::from(""),
      display_name: String::from(""),
      following: vec![String::from(
        "12D3KooWHxU85q4JWsDXq4ZHjBCdjHHGL9wnMtqBMMgArkn6xcyz",
      )],
      meta: json!({}),
      posts: Vec::new(),
      publisher: String::from(publisher),
      timestamp: timestamp,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaResponse {
  pub data: Vec<u8>,
  pub ext: String,
  pub mime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeRequest {
  pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeResponse {
  pub ext: String,
  pub mime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
  pub body: String,
  pub files: Vec<String>,
  pub meta: Value,
  pub publisher: String,
  pub timestamp: i64,
}
impl Post {
  pub fn new(body: String, files: Vec<String>, meta: Value, publisher: String) -> Post {
    Post {
      body: body,
      files: files,
      meta: meta,
      publisher: publisher,
      timestamp: DateTime::timestamp_millis(&Utc::now()),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRequest {
  pub body: String,
  pub files: Vec<String>,
  pub meta: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
  pub cid: String,
  pub display_name: String,
  pub post: Post,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher {
  pub publisher: String,
}
