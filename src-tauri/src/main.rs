#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod identity;
use crate::identity::types::AppState;
use crate::identity::{initialize_database, wait_for_ipfs_id};

use ipfs_api::IpfsClient;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use tauri::{
  api::dialog::ask, async_runtime, CustomMenuItem, Event, GlobalShortcutManager, Manager,
  SystemTray, SystemTrayEvent, SystemTrayMenu,
};

#[cfg(target_os = "linux")]
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct IpfsID {
  data: String,
}

fn main() {
  tauri::Builder::default()
    // .on_page_load(|window, _| {
    //   let window_ = window.clone();
    //   window.listen("ipfs-id", move |event| {
    //     println!("got js-event with message '{:?}'", event.payload());
    //     let id = IpfsID {
    //       data: "ipfs-id".to_string(),
    //     };
    //     window_
    //       .emit("rust-event", Some(id))
    //       .expect("failed to emit");
    //   });
    // })
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
      identity::get_identity,
      identity::get_feed,
      identity::ipfs_get_post,
      identity::ipfs_id,
      identity::post,
      identity::wait_for_ipfs_id_cmd,
    ])
    .setup(|app| {
      let daemon_client = IpfsClient::default();
      let splashscreen_window = app.get_window("splash").unwrap();
      let main_window = app.get_window("main").unwrap();
      let app_handle = app.handle();

      tauri::async_runtime::spawn(async move {
        match identity::launch_ipfs_daemon(&daemon_client).await {
          Ok(iden) => {
            initialize_database(iden.clone()).await;
            let reply = IpfsID { data: iden.clone() };
            main_window
              .emit("ipfs-id", Some(reply))
              .expect("failed to emit");

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

      let ipfs_client = IpfsClient::default();
      tauri::async_runtime::block_on(async move {
        match wait_for_ipfs_id(&ipfs_client.clone()).await {
          Ok(id) => {
            println!("opening sqlite db @ {:?}", String::from(id.clone() + ".db"));
            let db_manager = SqliteConnectionManager::file(String::from(id.clone() + ".db"));
            let db_pool = r2d2::Pool::new(db_manager).unwrap();
            let ipfs_client = IpfsClient::default();
            let ipfs_id = match ipfs_client.id(None).await {
              Ok(id) => Ok(id.id),
              Err(err) => Err(String::new()),
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
              .emit("ipfs-id", Some(reply))
              .expect("failed to emit");
          }
          Err(e) => {
            eprintln!("failed to wait_for_ipfs_id: {:?}", e)
          }
        };
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
