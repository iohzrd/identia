// Copyright 2021-2022, iohzrd <iohzrd@protonmail.com>
// SPDX-License-Identifier: AGPL-3.0

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use chrono::{offset::Utc, DateTime};
use feed_rs::parser;
use ipfs_api_backend_hyper::{request::Add, Form, IpfsApi, IpfsClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::io::Cursor;
use std::path::Path;
use std::{fs, path::PathBuf};
use std::{thread, time::Duration};
use tauri;
use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};
use tauri::api::path::config_dir;
use tauri::api::process::Command;
use tauri::Icon;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_sql::{Migration, MigrationKind, TauriSql};
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
struct PostRequest {
  body: String,
  files: Vec<String>,
  meta: Value,
  timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostResponse {
  cid: String,
  files: Vec<String>,
}

#[tauri::command]
async fn post(request: PostRequest) -> PostResponse {
  println!("post");
  println!("{:?}", request);
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
  println!("{:?}", post);
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

#[derive(Debug, Serialize, Deserialize)]
struct ExternalIdentity {
  url: String, // href
  title: String,
  description: String,
  entries: Vec<ExternalEntry>, // authors: Vec<String>,
                               // timestamp: i64
}

#[derive(Debug, Serialize, Deserialize)]
struct ExternalEntry {
  id: String,
}

#[tauri::command]
async fn fetch_external(url: String) -> ExternalIdentity {
  println!("fetch_external");
  let client = ClientBuilder::new().build().unwrap();
  let response = client
    .send(
      HttpRequestBuilder::new("GET", url)
        .unwrap()
        .response_type(ResponseType::Text),
    )
    .await
    .unwrap();
  let data: &[u8] = &response.bytes().await.unwrap().data;
  let feed = parser::parse(data).unwrap();
  println!("{:#?}", feed);
  let entries = feed
    .entries
    .iter()
    .map(|entry| ExternalEntry {
      id: entry.id.clone(),
    })
    .collect::<Vec<_>>();

  ExternalIdentity {
    url: feed.links[0].href.clone(),
    title: feed.title.unwrap().content,
    description: feed.description.unwrap().content,
    entries: entries, // timestamp: chrono(feed.updated),
  }
}

fn identia_app_data_path() -> PathBuf {
  config_dir()
    .expect("Could not get config dir")
    .join("identia")
}

fn create_dir_if_necessary(path: PathBuf) {
  if let Err(_) = fs::read(path.clone()) {
    let _result = fs::create_dir(path);
  }
}

fn initialize_ipfs() {
  create_dir_if_necessary(identia_app_data_path());
  println!(
    "Initializing IPFS: {:?}",
    identia_app_data_path().into_os_string().to_str().unwrap()
  );
  let cmd = Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "init",
      "--repo-dir",
      identia_app_data_path().into_os_string().to_str().unwrap(),
    ])
    .output()
    .unwrap();
  format!("IPFS init: {:?}", cmd);
  println!("configuring IPFS");
  Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "--repo-dir",
      identia_app_data_path().into_os_string().to_str().unwrap(),
      "config",
      "--json",
      "API.HTTPHeaders.Access-Control-Allow-Origin",
      r#"["http://127.0.0.1:5001","tauri://localhost","https://tauri.localhost"]"#,
    ])
    .output()
    .unwrap();
  Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "--repo-dir",
      identia_app_data_path().into_os_string().to_str().unwrap(),
      "config",
      "--json",
      "API.HTTPHeaders.Access-Control-Allow-Methods",
      r#"["GET","POST","PUT"]"#,
    ])
    .output()
    .unwrap();
  Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "--repo-dir",
      identia_app_data_path().into_os_string().to_str().unwrap(),
      "config",
      "--json",
      "Addresses.Gateway",
      "/ip4/127.0.0.1/tcp/8080",
    ])
    .output()
    .unwrap();
  Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "--repo-dir",
      identia_app_data_path().into_os_string().to_str().unwrap(),
      "config",
      "--json",
      "Datastore.StorageMax",
      "1000GB",
    ])
    .output()
    .unwrap();
}

fn main() {
  tauri::Builder::default()
    .system_tray(
      SystemTray::new()
        .with_menu(SystemTrayMenu::new().add_item(CustomMenuItem::new("exit_app", "Quit"))),
    )
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
        window.set_focus().unwrap();
      }
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "exit_app" => {
          tauri::async_runtime::block_on(async move {
            println!("shutting down IPFS");
            let ipfs_client = IpfsClient::default();
            match ipfs_client.shutdown().await {
              Ok(ret) => {
                println!("IPFS successfully terminated: {:?}", ret);
                app.exit(0)
              }
              Err(e) => eprintln!("IPFS exited with: {:#?}", e),
            };
          });
        }
        #[cfg(target_os = "linux")]
        "icon_1" => app
          .tray_handle()
          .set_icon(Icon::File(PathBuf::from("../icons/icon.png")))
          .unwrap(),
        #[cfg(target_os = "macos")]
        "icon_1" => {
          app.tray_handle().set_icon_as_template(true).unwrap();
          app
            .tray_handle()
            .set_icon(Icon::Raw(include_bytes!("../icons/icon.png").to_vec()))
            .unwrap();
        }
        #[cfg(target_os = "windows")]
        "icon_1" => app
          .tray_handle()
          .set_icon(Icon::Raw(include_bytes!("../icons/icon.ico").to_vec()))
          .unwrap(),
        _ => {}
      },
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![post, fetch_external])
    .setup(|_app| {
      initialize_ipfs();
      tauri::async_runtime::spawn(async move {
        println!("Starting IPFS.");
        env::set_var(
          "IPFS_PATH",
          identia_app_data_path().into_os_string().to_str().unwrap(),
        );
        Command::new_sidecar("ipfs")
          .expect("failed to setup ipfs sidecar")
          .args(&[
            "daemon",
            "--repo-dir",
            identia_app_data_path().into_os_string().to_str().unwrap(),
            "--migrate=true",
          ])
          .spawn()
          .expect("Failed to spawn ipfs");
      });

      tauri::async_runtime::block_on(async move {
        let client = IpfsClient::default();
        let mut ready = false;
        let mut retries = 1;
        while !ready {
          match client.id(None).await {
            Ok(resp) => {
              println!("using id: {}", resp.id);
              ready = true;
            }
            Err(_err) => {
              if retries > 6000 {
                break;
              }
              retries += 1;
              thread::sleep(Duration::from_millis(10));
            }
          }
        }
      });

      Ok(())
    })
    .plugin({
      TauriSql::default().add_migrations(
        "sqlite:sqlite.db",
        vec![Migration {
          version: 1,
          description: "create tables",
          sql: include_str!("../migrations/1.sql"),
          kind: MigrationKind::Up,
        }],
      )
    })
    .run(tauri::generate_context!())
    .expect("error while running identia");
}
