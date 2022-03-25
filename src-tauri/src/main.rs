// Copyright 2021-2022, iohzrd <iohzrd@protonmail.com>
// SPDX-License-Identifier: AGPL-3.0

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use ipfs_api::{Form, IpfsApi, IpfsClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::io::Cursor;
use std::path::Path;
use std::{fs, path::PathBuf};
use std::{thread, time::Duration};
use tauri;
use tauri::api::path::config_dir;
use tauri::api::process::Command;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_sql::{Migration, MigrationKind, TauriSql};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
  body: String,
  files: Vec<String>,
  meta: Value,
  publisher: String,
  timestamp: i64,
}
impl Post {
  fn new(body: String, files: Vec<String>, meta: Value, publisher: String, timestamp: i64) -> Post {
    Post {
      body: body,
      files: files,
      meta: meta,
      publisher: publisher,
      timestamp: timestamp,
    }
  }
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
  post: Post,
}

#[tauri::command]
async fn post(post_request: PostRequest) -> PostResponse {
  println!("post");
  println!("{:?}", post_request);
  let ipfs_client = IpfsClient::default();
  let ipfs_id = match ipfs_client.id(None).await {
    Ok(id) => id.id,
    Err(_err) => String::from(""),
  };
  let file_paths: Vec<String> = post_request.files.clone();
  let mut file_names: Vec<String> = Vec::new();

  let add = ipfs_api::request::Add::builder()
    .wrap_with_directory(true)
    .build();
  let mut form = Form::default();

  for filepath in file_paths {
    let data = fs::read(filepath.clone()).expect("Something went wrong reading the file");
    let filename = String::from(Path::new(&filepath).file_name().unwrap().to_str().unwrap());
    file_names.push(filename.clone());
    form.add_reader_file("path", Cursor::new(data), filename);
  }

  let post = Post::new(
    post_request.body,
    file_names,
    post_request.meta,
    ipfs_id,
    post_request.timestamp,
  );
  println!("{:?}", post);
  let json = serde_json::to_vec(&post).unwrap();
  form.add_reader_file("path", Cursor::new(json), "post.json");

  let ipfs_client = IpfsClient::default();
  let cid = match ipfs_client.add_with_form(form, add).await {
    Ok(res) => {
      println!("res: {:?}", res);
      let mut cid = String::from("");
      for add in res {
        if add.name == String::from("") {
          cid = add.hash
        }
      }
      cid
    }
    Err(e) => {
      eprintln!("{:#?}", e);
      String::from("")
    }
  };

  let post_response = PostResponse {
    cid: cid,
    post: post,
  };
  post_response
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
  println!("Initializing IPFS");
  let cmd = Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "init",
      "-c",
      identia_app_data_path().into_os_string().to_str().unwrap(),
    ])
    .output()
    .unwrap();
  format!("IPFS init: {:?}", cmd);
  println!("configuring IPFS");
  Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "-c",
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
      "-c",
      identia_app_data_path().into_os_string().to_str().unwrap(),
      "config",
      "--json",
      "API.HTTPHeaders.Access-Control-Allow-Methods",
      r#"["GET","POST","PUT"]"#,
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
        "exit_app" => app.exit(0),
        #[cfg(target_os = "linux")]
        "icon_1" => app
          .tray_handle()
          .set_icon(tauri::TrayIcon::File(PathBuf::from("../icons/icon.png")))
          .unwrap(),
        #[cfg(target_os = "macos")]
        "icon_1" => {
          app.tray_handle().set_icon_as_template(true).unwrap();
          app
            .tray_handle()
            .set_icon(tauri::TrayIcon::Raw(
              include_bytes!("../icons/icon.png").to_vec(),
            ))
            .unwrap();
        }
        #[cfg(target_os = "windows")]
        "icon_1" => app
          .tray_handle()
          .set_icon(tauri::TrayIcon::Raw(
            include_bytes!("../icons/icon.ico").to_vec(),
          ))
          .unwrap(),
        _ => {}
      },
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![post])
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
            "-c",
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
