// Copyright 2021-2022, iohzrd <iohzrd@protonmail.com>
// SPDX-License-Identifier: AGPL-3.0

use std::{fs, path::PathBuf};
use tauri::api::path::config_dir;

pub fn identia_app_data_path() -> PathBuf {
  config_dir()
    .expect("Could not get config dir")
    .join("identia")
}

pub fn identia_db_file_path(id: String) -> PathBuf {
  identia_app_data_path().join(id + ".db")
}

pub fn create_initial_config_if_necessary() -> () {
  create_dir_if_necessary(identia_app_data_path());
}
pub fn create_db_file_if_necessary(id: String) -> () {
  create_file_if_necessary(identia_db_file_path(id));
}

fn create_dir_if_necessary(path: PathBuf) {
  if let Err(_) = fs::read(path.clone()) {
    let _result = fs::create_dir(path);
  }
}

fn create_file_if_necessary(path: PathBuf) {
  if let Err(_) = fs::read(path.clone()) {
    let _result = fs::write(path, Vec::new());
  }
}
