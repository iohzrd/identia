// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use ipfs_api::IpfsClient;
// use percent_encoding;
use std::{thread, time::Duration};
use tauri::api::process::Command;
use tauri::Manager;
// use tauri::WindowUrl;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let splashscreen_window = app.get_window("splash").unwrap();
      let main_window = app.get_window("main").unwrap();
      // let splashscreen_window = app.create_window(
      //   "splashscreen".into(),
      //   WindowUrl::default(),
      //   move |window_builder, webview_attributes| {
      //     (
      //       window_builder,
      //       webview_attributes.register_uri_scheme_protocol("tauri", move |url| {
      //         let path = url.replace("tauri://", "");
      //         let path = percent_encoding::percent_decode(path.as_bytes())
      //           .decode_utf8_lossy()
      //           .to_string();
      //         let data =
      //           tauri::async_runtime::block_on(async move { tokio::fs::read(path).await })?;
      //         Ok(data)
      //       }),
      //     )
      //   },
      // );

      tauri::async_runtime::spawn(async move {
        match launch_ipfs_daemon().await {
          Ok(()) => {
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
      Ok(())
    })
    // .invoke_handler(tauri::generate_handler![close_splashscreen])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

async fn launch_ipfs_daemon() -> Result<(), String> {
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

  let client = IpfsClient::default();
  match wait_for_ipfs_ready(&client).await {
    Ok(ready) => println!("ipfs ready: {:?}", ready),
    Err(e) => eprintln!("error waiting for ipfs: {}", e),
  }

  match client.id(None).await {
    Ok(id) => println!("id: {:?}", id.id),
    Err(e) => eprintln!("error getting id: {}", e),
  }

  Ok(())
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
