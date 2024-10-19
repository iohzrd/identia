// Copyright 2021-2022, iohzrd <iohzrd@protonmail.com>
// SPDX-License-Identifier: AGPL-3.0

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod ipfs;
mod webarchiver;
mod webfeed;
use ipfs::{post, repost_webfeed_entry};
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::env;
use std::fs;
use tauri;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;
use tauri::Manager;
use tauri::{
  menu::{Menu, MenuItem},
  tray::TrayIconBuilder,
};
use tauri_plugin_sql::{Builder, Migration, MigrationKind};
use webfeed::fetch_webfeed;
use webarchiver::archive_webpage;


async fn shutdown_ipfs() {
  println!("shutting down IPFS");
  let ipfs_client = IpfsClient::default();
  match ipfs_client.shutdown().await {
    Ok(ret) => println!("IPFS successfully terminated: {:?}", ret),
    Err(e) => eprintln!("IPFS exited with: {:#?}", e),
  };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_sql::Builder::new().build())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_http::init())
    .invoke_handler(tauri::generate_handler![
      archive_webpage,
      fetch_webfeed,
      post,
      repost_webfeed_entry
    ])
    .setup(|app| {
      let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
      let menu = Menu::with_items(app, &[&quit_i])?;
      
      let tray = TrayIconBuilder::new()
      .on_menu_event(|app, event| match event.id.as_ref() {
        "quit" => {
          tauri::async_runtime::block_on(async move {
            shutdown_ipfs().await;
          });
          app.exit(0);
        }
        _ => {}
      })
        .menu(&menu)
        .menu_on_left_click(true)
        .build(app)?;

      let config_dir = app.path().app_config_dir().expect("failed to get home dir");
      // 

      if let Err(_) = fs::read(config_dir.clone()) {
        let _result = fs::create_dir(config_dir.clone());
      }

      // println!("Initializing IPFS: {:?}", config_dir);
      // let output = tauri::async_runtime::block_on(async move { 
      //   &app.shell().command("ipfs").args(&["init", "--repo-dir", config_dir.to_str().unwrap()]).output().await.unwrap() 
      // });
      // println!("IPFS init: {:?}", output);
    
      // output =  app.shell().sidecar("ipfs").unwrap()
      //   .args(&[
      //     "--repo-dir",
      //     config_dir.to_str().unwrap(),
      //     "config",
      //     "--json",
      //     "API.HTTPHeaders.Access-Control-Allow-Origin",
      //     r#"["http://127.0.0.1:1420","http://127.0.0.1:3000","http://127.0.0.1:5001","http://localhost:1420","http://localhost:3000","http://localhost:5001","http://webui.ipfs.io.ipns.localhost:8080","https://tauri.localhost","https://webui.ipfs.io","tauri://localhost"]"#,
      //   ]).spawn().unwrap();
      // println!("configuring Access-Control-Allow-Origin: {:?}", output);
    
      // output = app.shell().sidecar("ipfs").unwrap()
      //   .args(&[
      //     "--repo-dir",
      //     config_dir.to_str().unwrap(),
      //     "config",
      //     "--json",
      //     "API.HTTPHeaders.Access-Control-Allow-Methods",
      //     r#"["GET","POST","PUT"]"#,
      //   ])
      //   .spawn()
      //   .unwrap();
      // println!("configuring Access-Control-Allow-Methods: {:?}", output);
    
      // output = app.shell().sidecar("ipfs").unwrap()
      //   .args(&[
      //     "--repo-dir",
      //     config_dir.to_str().unwrap(),
      //     "config",
      //     "--json",
      //     "Addresses.Gateway",
      //     r#""/ip4/127.0.0.1/tcp/8080""#,
      //   ])
      //   .spawn()
      //   .unwrap();
      // println!("configuring Addresses.Gateway: {:?}", output);
    
      // output = app.shell().sidecar("ipfs").unwrap()
      //   .args(&[
      //     "--repo-dir",
      //     config_dir.to_str().unwrap(),
      //     "config",
      //     "--json",
      //     "Datastore.StorageMax",
      //     r#""1000GB""#,
      //   ])
      //   .spawn()
      //   .unwrap();
      // println!("configuring StorageMax: {:?}", output);

      // 
      let (mut rx, mut _child) = app.shell().sidecar("ipfs").unwrap()
        .args(&[
          "daemon",
          "--repo-dir",
          config_dir.to_str().unwrap(),
          "--migrate=true",
          "--enable-pubsub-experiment",
        ])
        .spawn()
        .expect("Failed to spawn sidecar");
      tauri::async_runtime::spawn(async move {
        println!("Starting IPFS.");
        env::set_var("IPFS_PATH", config_dir.to_str().unwrap());
        // read events such as stdout
        while let Some(event) = rx.recv().await {
          if let CommandEvent::Stdout(line_bytes) = event {
            let _line = String::from_utf8_lossy(&line_bytes);
            // write to stdin
            _child.write("message from Rust\n".as_bytes()).unwrap();
          }
        }
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
