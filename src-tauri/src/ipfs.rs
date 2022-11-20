// Copyright 2021-2022, iohzrd <iohzrd@protonmail.com>
// SPDX-License-Identifier: AGPL-3.0

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::webfeed::FilteredEntry;
use ipfs_api_backend_hyper::{request::Add, Form, IpfsApi, IpfsClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use tauri;
use urlencoding::encode;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
  body: String,
  files: Vec<String>,
  meta: Value,
  publisher: String,
  timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRequest {
  body: String,
  files: Vec<String>,
  meta: Value,
  timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
  cid: String,
  files: Vec<String>,
}

#[tauri::command]
pub async fn post(request: PostRequest) -> PostResponse {
  println!("post");
  println!("{:#?}", request);
  let ipfs_client = IpfsClient::default();
  let ipfs_id = match ipfs_client.id(None).await {
    Ok(id) => id.id,
    Err(_err) => String::from(""),
  };
  let file_paths: Vec<String> = request.files.clone();
  let mut file_names: Vec<String> = Vec::new();

  let add = Add {
    wrap_with_directory: Some(true),
    ..Default::default()
  };

  let mut form = Form::default();

  for filepath in file_paths {
    let data = fs::read(filepath.clone()).expect("Something went wrong reading the file");
    // let filename = String::from(Path::new(&filepath).file_name().unwrap().to_str().unwrap());
    let filename: String =
      encode(Path::new(&filepath).file_name().unwrap().to_str().unwrap()).into_owned();
    file_names.push(filename.clone());
    form.add_reader_file("path", Cursor::new(data), filename);
  }

  let post = Post {
    body: request.body,
    files: file_names.clone(),
    meta: request.meta,
    publisher: ipfs_id,
    timestamp: request.timestamp,
  };
  println!("{:#?}", post);
  let json = serde_json::to_vec(&post).unwrap();
  form.add_reader_file("path", Cursor::new(json), "post.json");

  let mut cid = String::from("");
  let mut res_file_names: Vec<String> = Vec::new();
  let ipfs_client = IpfsClient::default();
  match ipfs_client.add_with_form(form, add).await {
    Ok(res) => {
      println!("res: {:?}", res);
      for add in res {
        if add.name == String::from("") {
          cid = add.hash
        } else {
          res_file_names.push(add.name);
        }
      }
    }
    Err(e) => {
      eprintln!("{:#?}", e);
    }
  };

  PostResponse {
    cid: cid,
    files: res_file_names,
  }
}

#[tauri::command]
pub async fn repost_webfeed_entry(entry: FilteredEntry) {
  println!("repost_webfeed_entry");
  println!("{:#?}", entry);
  let file_urls: Vec<String> = entry
    .media
    .iter()
    .filter_map(|m| m.content.iter().find_map(|c| c.url.clone()))
    .collect();
  println!("{:#?}", file_urls);
  // println!("{:#?}", file_paths);
}
