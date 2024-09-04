// Copyright 2021-2022, iohzrd <iohzrd@protonmail.com>
// SPDX-License-Identifier: AGPL-3.0

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod ipfs;
mod webfeed;
use ipfs::{post, repost_webfeed_entry};
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::env;
use std::{fs, path::PathBuf};
use tauri;
use tauri::api::path::config_dir;
use tauri::api::process::Command;
use tauri::Icon;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_sql::{Builder, Migration, MigrationKind};
use webfeed::fetch_webfeed;

fn identia_app_data_path() -> PathBuf {
  config_dir().unwrap().join("identia")
}

fn initialize_ipfs() {
  let path = identia_app_data_path();
  if let Err(_) = fs::read(path.clone()) {
    let _result = fs::create_dir(path.clone());
  }
  println!("Initializing IPFS: {:?}", path);
  let mut output = Command::new_sidecar("ipfs")
    .unwrap()
    .args(&["init", "--repo-dir", path.to_str().unwrap()])
    .output()
    .unwrap();
  println!("IPFS init: {:?}", output);

  output =  Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "--repo-dir",
      path.to_str().unwrap(),
      "config",
      "--json",
      "API.HTTPHeaders.Access-Control-Allow-Origin",
      r#"["http://localhost:1420","http://127.0.0.1:1430","http://127.0.0.1:5001","tauri://localhost","https://tauri.localhost"]"#,
    ]).output().unwrap();
  println!("configuring Access-Control-Allow-Origin: {:?}", output);

  output = Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "--repo-dir",
      path.to_str().unwrap(),
      "config",
      "--json",
      "API.HTTPHeaders.Access-Control-Allow-Methods",
      r#"["GET","POST","PUT"]"#,
    ])
    .output()
    .unwrap();
  println!("configuring Access-Control-Allow-Methods: {:?}", output);

  output = Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "--repo-dir",
      path.to_str().unwrap(),
      "config",
      "--json",
      "Addresses.Gateway",
      r#""/ip4/127.0.0.1/tcp/8080""#,
    ])
    .output()
    .unwrap();
  println!("configuring Addresses.Gateway: {:?}", output);

  output = Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "--repo-dir",
      path.to_str().unwrap(),
      "config",
      "--json",
      "Datastore.StorageMax",
      r#""1000GB""#,
    ])
    .output()
    .unwrap();
  println!("configuring StorageMax: {:?}", output);
}

async fn shutdown_ipfs() {
  println!("shutting down IPFS");
  let ipfs_client = IpfsClient::default();
  match ipfs_client.shutdown().await {
    Ok(ret) => println!("IPFS successfully terminated: {:?}", ret),
    Err(e) => eprintln!("IPFS exited with: {:#?}", e),
  };
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
            shutdown_ipfs().await;
          });
          tauri::AppHandle::exit(app, 0);
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
    .invoke_handler(tauri::generate_handler![
      fetch_webfeed,
      post,
      repost_webfeed_entry
    ])
    .setup(|_app| {
      initialize_ipfs();
      let path = identia_app_data_path();
      tauri::async_runtime::spawn(async move {
        println!("Starting IPFS.");
        env::set_var("IPFS_PATH", path.to_str().unwrap());
        Command::new_sidecar("ipfs")
          .expect("failed to setup ipfs sidecar")
          .args(&[
            "daemon",
            "--repo-dir",
            path.to_str().unwrap(),
            "--migrate=true",
            "--enable-pubsub-experiment",
          ])
          .spawn()
          .expect("Failed to spawn ipfs");
      });
      Ok(())
    })
    .plugin({
      Builder::default()
        .add_migrations(
          "sqlite:sqlite.db",
          vec![
            Migration {
              version: 1,
              description: "create tables",
              sql: include_str!("../migrations/1.sql"),
              kind: MigrationKind::Up,
            },
            Migration {
              version: 2,
              description: "create comments table",
              sql: include_str!("../migrations/2.sql"),
              kind: MigrationKind::Up,
            },
            Migration {
              version: 3,
              description: "create topics table",
              sql: include_str!("../migrations/3.sql"),
              kind: MigrationKind::Up,
            },
          ],
        )
        .build()
    })
    .run(tauri::generate_context!())
    .expect("error while running identia");
}
