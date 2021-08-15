// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![allow(
    // Clippy bug: https://github.com/rust-lang/rust-clippy/issues/7422
    clippy::nonstandard_macro_braces,
)]

use tauri::{
  api::process::{Command},
};

fn main() {
  tauri::Builder::default()
    .setup(|_app| {
      // tauri::async_runtime::spawn(async move {
      tauri::async_runtime::block_on(async move {
        match launch_ipfs_daemon().await {
          Ok(()) => (),
          Err(_err) => {
            // log::error!("There was an error launching holochain: {:?}", err);
          }
        }
        // log::info!("Launch setup successful")
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

async fn launch_ipfs_daemon() -> Result<(), String> {
  // config::create_initial_config_if_necessary();
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
  Ok(())
}