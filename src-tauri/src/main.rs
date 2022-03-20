// Copyright 2021-2022, iohzrd <iohzrd@protonmail.com>
// SPDX-License-Identifier: AGPL-3.0

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use ipfs_api::{IpfsApi, IpfsClient};
use std::env;
use std::{fs, path::PathBuf};
use std::{thread, time::Duration};
use tauri::api::path::config_dir;
use tauri::api::process::Command;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_sql::{Migration, MigrationKind, TauriSql};

pub fn identia_app_data_path() -> PathBuf {
  config_dir()
    .expect("Could not get config dir")
    .join("identia")
}

fn create_dir_if_necessary(path: PathBuf) {
  if let Err(_) = fs::read(path.clone()) {
    let _result = fs::create_dir(path);
  }
}

pub fn initialize_ipfs() {
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

pub async fn get_ipfs_id(client: &IpfsClient) -> Result<String, String> {
  match client.id(None).await {
    Ok(id) => Ok(id.id),
    Err(err) => Err(err.to_string()),
  }
}

fn main() {
  // let mut id = "";
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
    .invoke_handler(tauri::generate_handler![])
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
              // id = &resp.id.as_str();
              // let f = || {
              //   id = resp.id.as_str();
              // };
              // f();
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
        // format!("sqlite:{}.db", id).as_str(),
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
