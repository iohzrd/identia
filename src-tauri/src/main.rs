// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

#[cfg(target_os = "linux")]
use std::path::PathBuf;

// use percent_encoding;
use ipfs_api::IpfsClient;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use serde::Deserialize;
use serde::Serialize;
use std::{thread, time::Duration};
use tauri::api::process::Command;
use tauri::{
  api::dialog::ask, async_runtime, CustomMenuItem, Event, GlobalShortcutManager, Manager,
  SystemTray, SystemTrayEvent, SystemTrayMenu, WindowBuilder, WindowUrl,
};
#[derive(Serialize)]
struct Reply {
  data: String,
}

fn main() {
  tauri::Builder::default()
    .on_page_load(|window, _| {
      let window_ = window.clone();
      window.listen("ipfs-id", move |event| {
        println!("got js-event with message '{:?}'", event.payload());
        let reply = Reply {
          data: "ipfs-id".to_string(),
        };

        window_
          .emit("rust-event", Some(reply))
          .expect("failed to emit");
      });
    })
    .system_tray(
      SystemTray::new().with_menu(
        SystemTrayMenu::new()
          .add_item(CustomMenuItem::new("toggle", "Toggle"))
          .add_item(CustomMenuItem::new("icon_1", "Tray Icon 1"))
          .add_item(CustomMenuItem::new("exit_app", "Quit")),
      ),
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
      SystemTrayEvent::MenuItemClick { id, .. } => {
        let item_handle = app.tray_handle().get_item(&id);
        match id.as_str() {
          "exit_app" => app.exit(0),
          "toggle" => {
            let window = app.get_window("main").unwrap();
            let new_title = if window.is_visible().unwrap() {
              window.hide().unwrap();
              "Show"
            } else {
              window.show().unwrap();
              "Hide"
            };
            item_handle.set_title(new_title).unwrap();
          }
          #[cfg(target_os = "linux")]
          "icon_1" => app
            .tray_handle()
            .set_icon(tauri::Icon::File(PathBuf::from("../icons/icon.png")))
            .unwrap(),
          #[cfg(target_os = "macos")]
          "icon_1" => {
            app.tray_handle().set_icon_as_template(true).unwrap();
            app
              .tray_handle()
              .set_icon(tauri::Icon::Raw(
                include_bytes!("../icons/icon.png").to_vec(),
              ))
              .unwrap();
          }
          #[cfg(target_os = "windows")]
          "icon_1" => app
            .tray_handle()
            .set_icon(tauri::Icon::Raw(
              include_bytes!("../icons/icon.ico").to_vec(),
            ))
            .unwrap(),
          _ => {}
        }
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      cmd::ipfs_id,
      cmd::log_operation,
      cmd::request_test_identity,
      cmd::ipfs_get_post
    ])
    .setup(|app| {
      let client = IpfsClient::default();
      let splashscreen_window = app.get_window("splash").unwrap();
      let main_window = app.get_window("main").unwrap();

      tauri::async_runtime::spawn(async move {
        match launch_ipfs_daemon(&client.clone()).await {
          Ok(iden) => {
            let reply = Reply { data: iden.clone() };
            main_window
              .emit("ipfs-id", Some(reply))
              .expect("failed to emit");

            // conn = Connection::open(iden.clone() + ".db");
            splashscreen_window.close().unwrap();
            main_window.show().unwrap();
          }
          Err(err) => {
            // log::error!("There was an error launching ipfs: {:?}", err);
            eprintln!("There was an error launching ipfs: {:?}", err);
          }
        }
        // log::info!("Launch setup successful")
        println!("Launch setup successful")
      });

      tauri::async_runtime::spawn(async move {
        let conn = Connection::open("test_spawn.db");
        // log::info!("Launch setup successful")
        println!("Launch setup successful")
      });

      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while building tauri application")
    .run(|app_handle, e| match e {
      // Application is ready (triggered only once)
      Event::Ready => {
        let app_handle = app_handle.clone();
        // launch a new thread so it doesnt block any channel
        async_runtime::spawn(async move {
          let app_handle = app_handle.clone();
          app_handle
            .global_shortcut_manager()
            .register("CmdOrCtrl+1", move || {
              let app_handle = app_handle.clone();
              let window = app_handle.get_window("main").unwrap();
              window.set_title("New title!").unwrap();
            })
            .unwrap();
        });
      }

      // Triggered when a window is trying to close
      Event::CloseRequested { label, api, .. } => {
        let app_handle = app_handle.clone();
        let window = app_handle.get_window(&label).unwrap();
        // use the exposed close api, and prevent the event loop to close
        api.prevent_close();
        // ask the user if he wants to quit
        // we need to run this on another thread because this is the event loop callback handler
        // and the dialog API needs to communicate with the event loop.
        std::thread::spawn(move || {
          ask(
            Some(&window),
            "Tauri API",
            "Are you sure that you want to close this window?",
            move |answer| {
              if answer {
                app_handle.get_window(&label).unwrap().close().unwrap();
              }
            },
          );
        });
      }

      // Keep the event loop running even if all windows are closed
      // This allow us to catch system tray events when there is no window
      Event::ExitRequested { api, .. } => {
        api.prevent_exit();
      }

      _ => {}
    })
}

async fn launch_ipfs_daemon(client: &IpfsClient) -> Result<String, String> {
  // config::create_initial_config_if_necessary();
  println!("Starting IPFS.");
  Command::new_sidecar("ipfs")
    .or(Err(String::from("Can't find ipfs binary")))?
    .args(&[
      "daemon",
      // config::conductor_config_path()
      //   .into_os_string()
      //   .to_str()
      //   .unwrap(),
    ])
    .spawn()
    .map_err(|err| format!("Failed to execute ipfs: {:?}", err))?;

  match wait_for_ipfs_ready(&client).await {
    Ok(ready) => println!("ipfs ready: {:?}", ready),
    Err(e) => eprintln!("error waiting for ipfs: {}", e),
  }

  match get_ipfs_id(&client).await {
    Ok(id) => Ok(id),
    Err(e) => Err(e),
  }
}

async fn wait_for_ipfs_ready(client: &IpfsClient) -> Result<bool, bool> {
  // A counter variable
  let mut ready = false;
  let mut retries = 1;
  // Loop while `n` is less than 101
  while !ready {
    match client.id(None).await {
      Ok(_id) => {
        ready = true;
      }
      Err(_err) => {
        if retries > 300 {
          // Err()
          break;
        }
        retries += 1;
        thread::sleep(Duration::from_millis(100));
      }
    }
  }

  Ok(ready)
}

async fn get_ipfs_id(client: &IpfsClient) -> Result<String, String> {
  match client.id(None).await {
    Ok(id) => Ok(id.id),
    Err(err) => Err(err.to_string()),
  }
}
