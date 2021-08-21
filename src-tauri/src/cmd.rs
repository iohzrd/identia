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

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
  id: i32,
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
  aux: Vec<String>,
  body: String,
  files: Vec<String>,
  files_root: String,
  meta: Vec<String>,
  post_cid: String,
  publisher: String,
  ts: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
  body: String,
  cid: String,
  from: String,
  kind: String,
  in_response_to: String,
  topic: String,
  ts: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityAuxObj {
  key: String,
  value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
  aux: Vec<IdentityAuxObj>,
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
pub fn request_test_identity() -> Identity {
  let test_aux_object = IdentityAuxObj {
    key: "BTC".to_string(),
    value: "1T8mM7TDWBcxKF5ZZy7B58adMsBgxivr1".to_string(),
  };
  let test_identity_object = Identity {
    aux: vec![test_aux_object],
    av: "".to_string(),
    dn: "iohzrd".to_string(),
    following: vec![
      "12D3KooWDED1CudLX9sdi1qBzy5tHS4Xi2Mpk45E5wrqteri1R8z".to_string(),
      "Qmb4zrL17TtLGnaLFuUQC4TmaVbizEfVbDnnSzNLxkZ3Zp".to_string(),
      "12D3KooWJd8q6Q9seVVA8HmSjqBN13kZaVZd4GtMxFha3VqiGfG9".to_string(),
    ],
    meta: vec![],
    posts: vec![
      "QmcoD56GRcE3eZZ3sFi91Ym98ZhvALxdtgPWmydRfX3sFx".to_string(),
      "QmQXHdVMMi45tDUEfNuaXMBbyeP1cAWjH7pf5V8ZPFStuj".to_string(),
      "QmRvi98hynA4qetsb9MG1HDQcLad8x9MNmGpqQFNBz59Bd".to_string(),
      "QmRF7xnqSJwRvmeg26TRFrr8xa9cV6Cm1nG68Wh112aF1X".to_string(),
      "QmW5wv3HR6LEZ6TXh6W6JkwTaALdpfvCRtEqcA2iQrdfEv".to_string(),
      "QmUvQAUr6zHQ22YMsbtoKeyGujm8qrXbe8ZLMdNy8YwzZf".to_string(),
      "QmcEZDkisuhqMfgeFxsms9syJ4b6cPhfKHDnZ3G2mH7PLV".to_string(),
      "QmVkwv3zGJx15wbHAfDAJ9ErjrvBGwP1cBo99t1mMben3Z".to_string(),
      "QmVDB1cfs3m93yBZNGoS6ujsMZkDb8ySHJtF3CmGCGcyQx".to_string(),
      "QmY4X8RtuuNig6BjJrLHeF4zURVAFFQPbLfJ4gfRHbm2wX".to_string(),
      "QmPkVnSjBa4b7qLUUYRwhwHJunoTFWwhvENxQB8tnrbyaH".to_string(),
      "QmWEvWXpcp9JSZ2taLhY8eby2CaBUTAojpXtJsfTab11Bh".to_string(),
      "QmNtN4TVvx2XL3f1BNB4v5wH5yeDwrNkZap9a1r6F6KQBM".to_string(),
      "QmQW72f51MRFj9PaJnLPcUWkZXRMcQVctf1ExJXrU3wWRs".to_string(),
    ],
    publisher: "12D3KooWDED1CudLX9sdi1qBzy5tHS4Xi2Mpk45E5wrqteri1R8z".to_string(),
    ts: DateTime::timestamp(&Utc::now()),
  };
  test_identity_object.into()
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
