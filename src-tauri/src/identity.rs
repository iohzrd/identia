use chrono::{offset::Utc, DateTime, NaiveDate};
use futures::TryStreamExt;
use ipfs_api::IpfsClient;
use rusqlite::{params, Connection, Result};
use rusqlite_migration::{Migrations, M};
use serde_json::{from_slice, json};
use std::{thread, time::Duration};
use tauri::api::process::Command;
use tauri::command;

use crate::identity::types::{Identity, Publisher};

pub mod types;

// #[command]
// pub fn log_operation(event: String, payload: Option<String>) {
//   println!("{} {:?}", event, payload);
// }

pub async fn identity_in_db(publisher: String) -> Result<bool> {
  let mut in_db = false;
  let conn = Connection::open("test.db")?;
  let mut stmt = conn.prepare("SELECT publisher FROM identity where publisher = ?")?;
  let identities = stmt.query_map(params![&publisher], |row| {
    Ok(Publisher {
      publisher: row.get(0)?,
    })
  })?;

  for iden in identities {
    let iden = iden.unwrap();
    if iden.publisher == String::from(publisher.clone()) {
      in_db = true;
    }
  }

  Ok(in_db)
}

pub fn get_identity_template(publisher: String) -> Identity {
  return Identity {
    aux: json!([]),
    av: "".to_string(),
    dn: "".to_string(),
    following: json!([String::from(publisher.clone())]),
    meta: json!([]),
    posts: json!([]),
    publisher: String::from(publisher.clone()),
    ts: DateTime::timestamp_millis(&Utc::now()),
  };
}

// pub async fn get_identity_db(publisher: String) -> Result<Identity> {
//   let conn = Connection::open("test.db")?;

//   let mut stmt = conn.prepare(
//     "SELECT aux,av,dn,following,meta,posts,publisher,ts FROM identity where publisher = ?",
//   )?;
//   let identities = stmt.query_map(params![&publisher], |row| {
//     Ok(Identity {
//       aux: row.get(0)?,
//       av: row.get(1)?,
//       dn: row.get(2)?,
//       following: row.get(3)?,
//       meta: row.get(4)?,
//       posts: row.get(5)?,
//       publisher: row.get(6)?,
//       ts: row.get(7)?,
//     })
//   })?;

//   for iden in identities {
//     let iden = iden.unwrap();
//     if iden.publisher == String::from(publisher.clone()) {
//       identity_in_db = true;
//       identity = iden;
//       // Ok(identity)
//     }
//   }

//   if !identity_in_db {
//     identity = types::Identity {
//       aux: json!([]),
//       av: "".to_string(),
//       dn: "".to_string(),
//       following: json!([String::from(publisher.clone())]),
//       meta: json!([]),
//       posts: json!([]),
//       publisher: String::from(publisher.clone()),
//       ts: DateTime::timestamp(&Utc::now()),
//     };
//   }

//   Ok(identity)
// }

// pub async fn get_identity_ipfs(publisher: String) -> Result<Identity> {
//   let client = IpfsClient::default();
// }

#[command]
pub async fn get_identity(publisher: String) -> types::Identity {
  let identity_object = types::Identity {
    aux: json!([]),
    av: String::from(""),
    dn: String::from(""),
    following: json!([String::from(publisher.clone())]),
    meta: json!([]),
    posts: json!([]),
    publisher: String::from(publisher.clone()),
    ts: DateTime::timestamp(&Utc::now()),
  };
  identity_object.into()
}

#[command]
pub fn request_test_identity() -> types::Identity {
  let test_aux_object = types::AuxObj {
    key: "BTC".to_string(),
    value: "1T8mM7TDWBcxKF5ZZy7B58adMsBgxivr1".to_string(),
  };
  let test_identity_object = types::Identity {
    aux: json!([test_aux_object]),
    av: "".to_string(),
    dn: "iohzrd".to_string(),
    following: json!([
      "12D3KooWDED1CudLX9sdi1qBzy5tHS4Xi2Mpk45E5wrqteri1R8z",
      "Qmb4zrL17TtLGnaLFuUQC4TmaVbizEfVbDnnSzNLxkZ3Zp".to_string(),
      "12D3KooWJd8q6Q9seVVA8HmSjqBN13kZaVZd4GtMxFha3VqiGfG9".to_string(),
    ]),
    meta: json!([]),
    posts: json!([
      "QmcoD56GRcE3eZZ3sFi91Ym98ZhvALxdtgPWmydRfX3sFx".to_string(),
      "QmQXHdVMMi45tDUEfNuaXMBbyeP1cAWjH7pf5V8ZPFStuj".to_string(),
      "QmRvi98hynA4qetsb9MG1HDQcLad8x9MNmGpqQFNBz59Bd".to_string(),
      "QmRF7xnqSJwRvmeg26TRFrr8xa9cV6Cm1nG68Wh112aF1X".to_string(),
      "QmW5wv3HR6LEZ6TXh6W6JkwTaALdpfvCRtEqcA2iQrdfEv".to_string(),
      "QmUvQAUr6zHQ22YMsbtoKeyGujm8qrXbe8ZLMdNy8YwzZf".to_string(),
      "QmcEZDkisuhqMfgeFxsms9syJ4b6cPhfKHDnZ3G2mH7PLV".to_string(),
      "QmVkwv3zGJx15wbHAfDAJ9ErjrvBGwP1cBo99t1mMben3Z".to_string(),
      "QmVDB1cfs3m93yBZNGoS6ujsMZkDb8ySHJtF3CmGCGcyQx".to_string(),
      "QmY4X8RtuuNig6BjJrLHeF4zURVAFFQPbLfJ4gfRHbm2wX".to_string(),
      "QmPkVnSjBa4b7qLUUYRwhwHJunoTFWwhvENxQB8tnrbyaH".to_string(),
      "QmWEvWXpcp9JSZ2taLhY8eby2CaBUTAojpXtJsfTab11Bh".to_string(),
      "QmNtN4TVvx2XL3f1BNB4v5wH5yeDwrNkZap9a1r6F6KQBM".to_string(),
      "QmQW72f51MRFj9PaJnLPcUWkZXRMcQVctf1ExJXrU3wWRs".to_string(),
    ]),
    publisher: "12D3KooWDED1CudLX9sdi1qBzy5tHS4Xi2Mpk45E5wrqteri1R8z".to_string(),
    ts: DateTime::timestamp(&Utc::now()),
  };
  test_identity_object.into()
}

#[command]
pub async fn ipfs_get_post(cid: String) -> Option<types::PostResponse> {
  let mut cid_json: String = cid.clone();
  if !cid_json.contains("/post.json") {
    cid_json.push_str("/post.json");
  }
  let client = IpfsClient::default();

  match client
    .cat(&cid_json)
    .map_ok(|chunk| chunk.to_vec())
    .try_concat()
    .await
  {
    Ok(res) => {
      let post_response = types::PostResponse {
        cid: cid.clone(),
        post: from_slice(&res).unwrap(),
      };
      println!("{:#?}", post_response);
      Some(post_response)
    }
    Err(e) => {
      eprintln!("{:#?}", e);
      None
    }
  }
  // post.postCid = Some(cid.clone());
  // post.into()
}

#[command]
pub async fn ipfs_id() -> String {
  let client = IpfsClient::default();
  let mut iden = String::from("");
  match client.id(None).await {
    Ok(id) => iden = id.id,
    Err(e) => eprintln!("error getting id: {}", e),
  }
  iden.into()
}

pub async fn initialize_database(publisher: &String) -> Result<()> {
  let mut identity_exists = false;
  println!("initialize_database: {:?}", &publisher);

  let mut conn = Connection::open("test.db")?;

  let create_identity_table = "create table if not exists identity (
      aux         text,
      av          text,
      dn          text,
      following   text,
      meta        text,
      posts       text,
      publisher   text primary key,
      ts          int
    )";

  let create_post_table = "create table if not exists identity (
        id          integer primary key,
        aux         text,
        av          text,
        dn          text,
        following   text,
        meta        text,
        posts       text,
        publisher   text,
        ts          int
      )";

  let migrations = Migrations::new(vec![
    M::up(create_identity_table),
    M::up(create_post_table),
    // In the future, add more migrations here
  ]);

  migrations.to_latest(&mut conn).unwrap();

  let mut stmt = conn.prepare("SELECT publisher FROM identity where publisher = ?")?;
  let identities = stmt.query_map(params![&publisher], |row| {
    Ok(Identity {
      aux: row.get(0)?,
      av: row.get(1)?,
      dn: row.get(2)?,
      following: row.get(3)?,
      meta: row.get(4)?,
      posts: row.get(5)?,
      publisher: row.get(6)?,
      ts: row.get(7)?,
    })
  })?;

  //   println!("identities {:?}", &identities.);

  for identity in identities {
    if identity.unwrap().publisher == String::from(publisher) {
      identity_exists = true;
    }
  }
  if !identity_exists {
    let me = types::Identity {
      aux: json!([]),
      av: "".to_string(),
      dn: "".to_string(),
      following: json!([String::from(publisher)]),
      meta: json!([]),
      posts: json!([]),
      publisher: String::from(publisher),
      ts: DateTime::timestamp(&Utc::now()),
    };
    conn.execute(
      "INSERT INTO identity (aux,av,dn,following,meta,posts,publisher,ts) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
      params![
          me.aux,
          me.av,
          me.dn,
          me.following,
          me.meta,
          me.posts,
          me.publisher,
          me.ts,
      ],
  )?;
  }

  Ok(())
}

pub async fn launch_ipfs_daemon(client: &IpfsClient) -> Result<String, String> {
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

pub async fn wait_for_ipfs_ready(client: &IpfsClient) -> Result<bool, bool> {
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

pub async fn get_ipfs_id(client: &IpfsClient) -> Result<String, String> {
  match client.id(None).await {
    Ok(id) => Ok(id.id),
    Err(err) => Err(err.to_string()),
  }
}
