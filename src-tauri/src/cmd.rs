// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use chrono::prelude::{DateTime, Utc};
use ipfs_api::request::Id;
use ipfs_api::IpfsClient;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tauri::command;

#[derive(Debug, Deserialize)]
pub struct RequestBody {
  id: i32,
  name: String,
}

#[derive(Debug, Serialize)]
pub struct Identity {
  aux: Vec<String>,
  av: String,
  dn: String,
  following: Vec<String>,
  meta: Vec<String>,
  posts: Vec<String>,
  publisher: String,
  ts: i64,
}

#[command]
pub fn log_operation(event: String, payload: Option<String>) {
  println!("{} {:?}", event, payload);
}

#[command]
pub fn request_test_obj() -> Identity {
  let me = Identity {
    aux: vec![],
    av: "".to_string(),
    dn: "iohzrd".to_string(),
    following: vec![],
    meta: vec![],
    posts: vec![],
    publisher: "asdf1234".to_string(),
    ts: DateTime::timestamp(&Utc::now()),
  };
  me.into()
}

#[command]
pub fn perform_request(endpoint: String, body: RequestBody) -> String {
  println!("{} {:?}", endpoint, body);
  "message response".into()
}

#[command]
pub async fn ipfs_id() -> String {
  let client = IpfsClient::default();
  let mut iden = String::from("");
  match client.id(None).await {
    Ok(id) => iden = id.id,
    Err(e) => eprintln!("error getting id: {}", e),
  }
  iden.into()
}

// #[derive(Debug)]
// struct Identity {
//   id: String,
//   name: String,
//   data: Option<Vec<u8>>,
// }

// #[command]
// pub async fn db_query(id: String) -> Identity {
//   let conn = Connection::open(id + ".db")?;
//   conn.execute(
//     "CREATE TABLE person (
//               id              INTEGER PRIMARY KEY,
//               name            TEXT NOT NULL,
//               data            BLOB
//               )",
//     [],
//   )?;
//   "iden".into()
// }
