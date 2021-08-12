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

fn main() {
  let ctx = tauri::generate_context!();
  tauri::Builder::default()
    // .register_global_uri_scheme_protocol(String::from("file://"), uri_handler)
    .run(ctx)
    .expect("error while running tauri application");
}
