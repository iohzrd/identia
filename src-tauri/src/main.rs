// Copyright 2021-2022, iohzrd <iohzrd@protonmail.com>
// SPDX-License-Identifier: AGPL-3.0

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod config;
mod identity;
use crate::identity::types::AppState;
use crate::identity::{initialize_database, wait_for_ipfs_id};

use ipfs_api::{IpfsApi, IpfsClient};
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

#[cfg(target_os = "linux")]
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct IpfsID {
  data: String,
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
    .invoke_handler(tauri::generate_handler![
      identity::delete_post,
      identity::follow_publisher,
      identity::get_display_name_db,
      identity::get_identity,
      identity::get_identity_ipfs_cmd,
      identity::get_post_ipfs,
      identity::ipfs_id,
      identity::post,
      identity::query_posts,
      identity::repost,
      identity::update_feed,
      identity::update_identity_aux,
      identity::wait_for_ipfs_id_cmd,
    ])
    .setup(|app| {
      identity::initialize_ipfs();
      identity::initialize_ipfs_config();

      let daemon_client = IpfsClient::default();
      tauri::async_runtime::spawn(async move {
        match identity::launch_ipfs_daemon(&daemon_client).await {
          Ok(id) => {
            config::create_db_file_if_necessary(id.clone());
            let db_file_path = config::identia_db_file_path(id.clone());
            let _ = initialize_database(id.clone(), db_file_path).await;
          }
          Err(err) => {
            // log::error!("There was an error launching ipfs: {:?}", err);
            eprintln!("There was an error launching ipfs: {:?}", err);
          }
        }
        // log::info!("Launch setup successful")
        println!("Launch setup successful")
      });

      let app_handle = app.handle();
      let ipfs_client = IpfsClient::default();
      tauri::async_runtime::block_on(async move {
        match wait_for_ipfs_id(&ipfs_client.clone()).await {
          Ok(id) => {
            let db_file_path = config::identia_db_file_path(id.clone());
            println!("opening sqlite db @ {:?}", db_file_path);
            let db_manager = SqliteConnectionManager::file(db_file_path);
            let db_pool = r2d2::Pool::new(db_manager).unwrap();
            let ipfs_client = IpfsClient::default();
            let ipfs_id = match ipfs_client.id(None).await {
              Ok(id) => Ok(id.id),
              Err(_err) => Err(String::new()),
            }
            .unwrap();
            let main_window = app.get_window("main").unwrap();
            let state_manager = app_handle;
            let app_state = AppState {
              ipfs_id: ipfs_id,
              db_pool: db_pool,
              ipfs_client: ipfs_client,
            };
            state_manager.manage(app_state);

            let reply = IpfsID { data: id.clone() };
            main_window
              .emit("ipfs-id", Some(&reply))
              .expect("failed to emit");
          }
          Err(e) => {
            eprintln!("failed to wait_for_ipfs_id: {:?}", e)
          }
        };
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running identia");
}
