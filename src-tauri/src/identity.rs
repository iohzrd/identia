use chrono::{offset::Utc, DateTime};
use futures::TryStreamExt;
use ipfs_api::IpfsClient;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection, Result};
use rusqlite_migration::{Migrations, M};
use serde_json::{from_slice, json};
use std::{thread, time::Duration};
use tauri;
use tauri::api::process::Command;

use crate::identity::types::{AppState, AuxObj, Identity, PostResponse};

pub mod types;

// #[tauri::command]
// pub fn log_operation(event: String, payload: Option<String>) {
//   println!("{} {:?}", event, payload);
// }

const create_identity_table: &str = "
    create table if not exists identity (
    aux         text,
    av          text,
    dn          text,
    following   text,
    meta        text,
    posts       text,
    publisher   text primary key,
    ts          int
  )";

const create_post_table: &str = "
    create table if not exists identity (
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

pub async fn get_identity_ipfs(publisher: String) -> Option<Identity> {
  let client = IpfsClient::default();
  let mut identity_json: String = publisher.clone();
  if !identity_json.contains("/identity.json") {
    identity_json.push_str("/identity.json");
  }

  match client
    .cat(&identity_json)
    .map_ok(|chunk| chunk.to_vec())
    .try_concat()
    .await
  {
    Ok(res) => {
      let identity: Identity = from_slice(&res).unwrap();
      println!("identity: {:#?}", identity);
      Some(identity)
    }
    Err(e) => {
      eprintln!("{:#?}", e);
      None
    }
  }
}

// #[tauri::command]
// pub async fn ipfs_id(state: tauri::State<'_, AppState>) -> String {
//   // let ipfs_client = state.ipfs_client;
//   let iden = match state.ipfs_client.id(None).await {
//     Ok(id) => id.id,
//     Err(e) => String::from(""),
//   };
//   println!("test_function");
//   println!("{:?}", iden);
//   iden
// }

#[tauri::command]
pub async fn ipfs_id() -> Result<String, String> {
  let client = IpfsClient::default();
  let iden = match client.id(None).await {
    Ok(id) => id.id,
    Err(e) => {
      eprintln!("error: ipfs_id(): {}", e);
      "".into()
    }
  };
  Ok(iden)
}

pub async fn identity_in_db(conn: &Connection, publisher: String) -> Result<bool> {
  let mut in_db = false;
  let mut stmt = conn.prepare("SELECT publisher FROM identity where publisher = ?")?;
  // in_db = stmt.query_row(params![&publisher], |row| Ok(true))?;
  in_db = stmt.query_row(params![&publisher], |_| Ok(true))?;

  Ok(in_db)
}

pub async fn insert_new_identity(
  conn: PooledConnection<SqliteConnectionManager>,
  identity: Identity,
) -> Identity {
  conn.execute(
    "INSERT INTO identity (aux,av,dn,following,meta,posts,publisher,ts) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    &[
      &identity.aux,
      &identity.av,
      &identity.dn,
      &identity.following,
      &identity.meta,
      &identity.posts,
      &identity.publisher,
      &identity.ts,
    ],
  ).unwrap();
  identity
}

pub async fn get_identity_db(
  conn: PooledConnection<SqliteConnectionManager>,
  publisher: String,
) -> Result<Identity> {
  let mut stmt = conn.prepare(
    "SELECT aux,av,dn,following,meta,posts,publisher,ts FROM identity where publisher = ?",
  )?;
  let identity = stmt.query_row(params![&publisher], |row| {
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

  Ok(identity)
}

#[tauri::command]
pub async fn get_identity(
  state: tauri::State<'_, AppState>,
  publisher: String,
) -> Result<Identity, Identity> {
  // let db_pool = state.db_pool.clone();
  let conn = state.db_pool.get().unwrap();
  let result = get_identity_db(conn, publisher.clone()).await;
  let identity = match result {
    Ok(i) => i,
    Err(_) => {
      let conn2 = state.db_pool.get().unwrap();
      insert_new_identity(conn2, Identity::new(publisher.clone())).await
    }
  };
  Ok(identity)
}

#[tauri::command]
pub async fn test_managed_state(state: tauri::State<'_, AppState>) -> Result<String, String> {
  let db_manager = state.db_pool.get().unwrap();
  let iden = match state.ipfs_client.id(None).await {
    Ok(id) => {
      println!("test_managed_state(): {}", id.id);
      id.id
    }
    Err(e) => {
      eprintln!("error: test_managed_state(): {}", e);
      "".into()
    }
  };
  println!("test_managed_state(): {}", iden);
  Ok(iden)
}

#[tauri::command]
pub fn request_test_identity() -> Identity {
  let test_aux_object = AuxObj {
    key: String::from("BTC"),
    value: String::from("1T8mM7TDWBcxKF5ZZy7B58adMsBgxivr1"),
  };
  let test_identity_object = Identity {
    aux: json!([test_aux_object]),
    av: json!(""),
    dn: json!("iohzrd"),
    following: json!([
      "12D3KooWDED1CudLX9sdi1qBzy5tHS4Xi2Mpk45E5wrqteri1R8z",
      "Qmb4zrL17TtLGnaLFuUQC4TmaVbizEfVbDnnSzNLxkZ3Zp",
      "12D3KooWJd8q6Q9seVVA8HmSjqBN13kZaVZd4GtMxFha3VqiGfG9",
    ]),
    meta: json!([]),
    posts: json!([
      "QmcoD56GRcE3eZZ3sFi91Ym98ZhvALxdtgPWmydRfX3sFx",
      "QmQXHdVMMi45tDUEfNuaXMBbyeP1cAWjH7pf5V8ZPFStuj",
      "QmRvi98hynA4qetsb9MG1HDQcLad8x9MNmGpqQFNBz59Bd",
      "QmRF7xnqSJwRvmeg26TRFrr8xa9cV6Cm1nG68Wh112aF1X",
      "QmW5wv3HR6LEZ6TXh6W6JkwTaALdpfvCRtEqcA2iQrdfEv",
      "QmUvQAUr6zHQ22YMsbtoKeyGujm8qrXbe8ZLMdNy8YwzZf",
      "QmcEZDkisuhqMfgeFxsms9syJ4b6cPhfKHDnZ3G2mH7PLV",
      "QmVkwv3zGJx15wbHAfDAJ9ErjrvBGwP1cBo99t1mMben3Z",
      "QmVDB1cfs3m93yBZNGoS6ujsMZkDb8ySHJtF3CmGCGcyQx",
      "QmY4X8RtuuNig6BjJrLHeF4zURVAFFQPbLfJ4gfRHbm2wX",
      "QmPkVnSjBa4b7qLUUYRwhwHJunoTFWwhvENxQB8tnrbyaH",
      "QmWEvWXpcp9JSZ2taLhY8eby2CaBUTAojpXtJsfTab11Bh",
      "QmNtN4TVvx2XL3f1BNB4v5wH5yeDwrNkZap9a1r6F6KQBM",
      "QmQW72f51MRFj9PaJnLPcUWkZXRMcQVctf1ExJXrU3wWRs",
    ]),
    publisher: json!("12D3KooWDED1CudLX9sdi1qBzy5tHS4Xi2Mpk45E5wrqteri1R8z"),
    ts: json!(DateTime::timestamp(&Utc::now())),
  };
  test_identity_object.into()
}

#[tauri::command]
pub async fn ipfs_get_post(cid: String) -> Option<PostResponse> {
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
      let post_response = PostResponse {
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

pub async fn initialize_database(publisher: &String) -> Result<()> {
  println!("initialize_database: {:?}", &publisher);
  let mut conn = Connection::open("test.db")?;
  let migrations = Migrations::new(vec![
    M::up(create_identity_table),
    M::up(create_post_table),
    // In the future, add more migrations here
  ]);
  migrations.to_latest(&mut conn).unwrap();

  let me = Identity::new(publisher.clone());
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

  // let mut stmt = conn.prepare("SELECT publisher FROM identity where publisher = ?")?;
  // let identities = stmt.query_map(params![&publisher], |row| {
  //   Ok(Identity {
  //     aux: row.get(0)?,
  //     av: row.get(1)?,
  //     dn: row.get(2)?,
  //     following: row.get(3)?,
  //     meta: row.get(4)?,
  //     posts: row.get(5)?,
  //     publisher: row.get(6)?,
  //     ts: row.get(7)?,
  //   })
  // })?;

  // println!("identities {:?}", identities);

  // for identity in identities {
  //   if identity.unwrap().publisher == String::from(publisher) {
  //     identity_exists = true;
  //   }
  // }
  // if !identity_exists {
  //   let me = types::Identity {
  //     aux: json!([]),
  //     av: String::from(""),
  //     dn: String::from(""),
  //     following: json!([String::from(publisher)]),
  //     meta: json!([]),
  //     posts: json!([]),
  //     publisher: String::from(publisher),
  //     ts: DateTime::timestamp(&Utc::now()),
  //   };
  //   conn.execute(
  //     "INSERT INTO identity (aux,av,dn,following,meta,posts,publisher,ts) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
  //     params![
  //         me.aux,
  //         me.av,
  //         me.dn,
  //         me.following,
  //         me.meta,
  //         me.posts,
  //         me.publisher,
  //         me.ts,
  //     ],
  // )?;
  // }

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
