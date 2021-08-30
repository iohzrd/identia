use chrono::{offset::Utc, DateTime};
use futures::TryStreamExt;
use ipfs_api::IpfsClient;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{named_params, params, Connection, Result};
use rusqlite_migration::{Migrations, M};
use serde_json::{from_slice, from_value, json, to_value};
use std::io::Cursor;
use std::{thread, time::Duration};
use tauri;
use tauri::api::process::Command;

pub mod migrations;
pub mod types;
use crate::identity::migrations::{create_identities_table, create_posts_table};
use crate::identity::types::{AppState, AuxObj, Identity, Post, PostRequest, PostResponse};

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

#[tauri::command]
pub async fn ipfs_id(state: tauri::State<'_, AppState>) -> Result<String, String> {
  let iden = match state.ipfs_client.id(None).await {
    Ok(id) => id.id,
    Err(e) => {
      eprintln!("error: ipfs_id(): {}", e);
      "".into()
    }
  };
  Ok(iden)
}

pub async fn identity_in_db(
  conn: PooledConnection<SqliteConnectionManager>,
  publisher: String,
) -> Result<bool, bool> {
  let mut in_db = false;
  let mut stmt = conn
    .prepare("SELECT publisher FROM identity where publisher = ?")
    .unwrap();
  in_db = stmt.query_row(params![&publisher], |_| Ok(true)).unwrap();
  Ok(in_db)
}

pub async fn db_insert_identity(
  conn: PooledConnection<SqliteConnectionManager>,
  identity: Identity,
) -> Identity {
  conn.execute(
    "INSERT INTO identities (aux,av,dn,following,meta,posts,publisher,ts) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    params![
      // &identity.aux,
      serde_json::to_value(&identity.aux).unwrap(),
      &identity.av,
      &identity.dn,
      // &identity.following,
      serde_json::to_value(&identity.following).unwrap(),
      // &identity.meta,
      serde_json::to_value(&identity.meta).unwrap(),
      // &identity.posts,
      serde_json::to_value(&identity.posts).unwrap(),
      &identity.publisher,
      &identity.ts,
    ],
  ).unwrap();
  identity
}

pub async fn db_update_identity(
  conn: PooledConnection<SqliteConnectionManager>,
  identity: Identity,
) -> Identity {
  let stmt = conn.prepare(
    "UPDATE identities SET aux=:aux, av=:av, dn=:dn, following=:following, meta=:meta, posts=:posts, publisher=:publisher, ts=:ts WHERE publisher = (:publisher)",
  );
  let mut s = match stmt {
    Ok(stmt) => {
      println!("stmt valid");
      stmt
    }
    Err(error) => {
      panic!("invalid sql in db_update_identity: {:?}", error)
    }
  };

  match s.execute(
    named_params! {":aux": serde_json::to_value(&identity.aux).unwrap(),
    ":av": &identity.av,
    ":dn": &identity.dn,
    ":following": serde_json::to_value(&identity.following).unwrap(),
    ":meta": serde_json::to_value(&identity.meta).unwrap(),
    ":posts": serde_json::to_value(&identity.posts).unwrap(),
    ":publisher": &identity.publisher,
    ":ts": &identity.ts},
  ) {
    Ok(i) => {
      println!("execute success: {:?}", i);
    }
    Err(e) => {
      eprintln!("execute failed: {:?}", e);
    }
  };
  identity
}

#[tauri::command]
pub async fn test_db_insert_identity(
  state: tauri::State<'_, AppState>,
  publisher: String,
) -> Result<Identity, Identity> {
  println!("test_db_insert_identity1");
  let conn = state.db_pool.get().unwrap();
  let identity = db_insert_identity(conn, Identity::new(publisher.clone())).await;

  Ok(identity)
}

pub async fn get_identity_db(
  conn: PooledConnection<SqliteConnectionManager>,
  publisher: String,
) -> Result<Identity> {
  let stmt = conn.prepare(
    "SELECT aux,av,dn,following,meta,posts,publisher,ts FROM identities where publisher = ?",
  );
  let mut s = match stmt {
    Ok(stmt) => stmt,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  };
  let identity = s
    .query_row(params![&publisher], |row| {
      Ok(Identity {
        // aux: row.get(0)?,
        aux: serde_json::from_value(row.get(0)?).unwrap(),
        av: row.get(1)?,
        dn: row.get(2)?,
        // following: row.get(3)?,
        following: serde_json::from_value(row.get(3)?).unwrap(),
        // meta: row.get(4)?,
        meta: serde_json::from_value(row.get(4)?).unwrap(),
        // posts: row.get(5)?,
        posts: serde_json::from_value(row.get(5)?).unwrap(),
        publisher: row.get(6)?,
        ts: row.get(7)?,
      })
    })
    .unwrap();
  println!("get_identity_db: {:?}", identity);
  Ok(identity)
}

#[tauri::command]
pub async fn get_identity(
  state: tauri::State<'_, AppState>,
  publisher: String,
) -> Result<Identity, Identity> {
  let conn = state.db_pool.get().unwrap();
  let result = get_identity_db(conn, publisher.clone()).await;
  let identity = match result {
    Ok(i) => i,
    Err(_) => {
      let conn2 = state.db_pool.get().unwrap();
      db_insert_identity(conn2, Identity::new(publisher.clone())).await
    }
  };
  Ok(identity)
}

pub async fn get_identity_internal(
  conn: PooledConnection<SqliteConnectionManager>,
  publisher: String,
) -> Result<Identity, Identity> {
  let result = get_identity_db(conn, publisher.clone()).await;
  let identity = match result {
    Ok(i) => i,
    Err(_) => Identity::new(publisher.clone()),
  };
  Ok(identity)
}

#[tauri::command]
pub async fn test_managed_state(state: tauri::State<'_, AppState>) -> Result<String, String> {
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
  let conn = state.db_pool.get().unwrap();
  let identity = match get_identity(state, iden.clone()).await {
    Ok(id) => id,
    Err(id) => id,
  };

  // let following: Vec<String> = serde_json::from_value(identity.following).unwrap();
  for publisher in identity.following {
    println!("{:?}", publisher.clone())
  }

  println!("test_managed_state(): {}", iden);
  Ok(iden)
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

// #[tauri::command]
// pub async fn ipfs_get_post(state: tauri::State<'_, AppState>, cid: String) -> Result<PostResponse> {
//   let mut cid_json: String = cid.clone();
//   if !cid_json.contains("/post.json") {
//     cid_json.push_str("/post.json");
//   }
//   let client = IpfsClient::default();
//   let post_response = match client
//     .cat(&cid_json)
//     .map_ok(|chunk| chunk.to_vec())
//     .try_concat()
//     .await
//   {
//     Ok(res) => PostResponse {
//       cid: cid.clone(),
//       post: from_slice(&res).unwrap(),
//     },

//     Err(e) => PostResponse {
//       post: Post::new(),
//       cid: String::from(""),
//     },
//   };
//   // post.postCid = Some(cid.clone());
//   // post.into()
//   Ok(post_response)
// }

#[tauri::command]
pub async fn get_feed(state: tauri::State<'_, AppState>) -> Vec<PostResponse> {
  let feed = vec![];
  let conn = state.db_pool.get().unwrap();
  let stmt =
    conn.prepare("SELECT aux,body,files,filesRoot,files_root,meta,publisher,ts FROM posts");
  let mut s = match stmt {
    Ok(stmt) => stmt,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  };

  feed.into()
}

#[tauri::command]
pub async fn post(
  state: tauri::State<'_, AppState>,
  postRequest: PostRequest,
) -> Result<PostResponse, PostResponse> {
  println!("post");
  println!("{:?}", postRequest.body);
  println!("{:?}", postRequest.files);
  let mut post = Post::new();
  post.body = postRequest.body;
  post.publisher = state.ipfs_id.clone();
  // post.files = request.files;
  println!("{:?}", post);

  let conn = state.db_pool.get().unwrap();
  let data = serde_json::to_string(&post).unwrap();
  let cursor = Cursor::new(data);
  let cid = match state.ipfs_client.add(cursor).await {
    Ok(res) => {
      println!("post success: {:?}", res.hash);
      res.hash
    }
    Err(e) => {
      eprintln!("{:#?}", e);
      String::from("")
    }
  };
  // post.postCid = Some(cid.clone());
  // post.into()

  conn.execute(
    "INSERT INTO posts (aux,body,files,files_cid,meta,publisher,ts) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    params![
      serde_json::to_value(&post.aux).unwrap(),
      &post.body,
      serde_json::to_value(&post.files).unwrap(),
      &post.files_cid,
      serde_json::to_value(&post.meta).unwrap(),
      &post.publisher,
      &post.ts,
    ],
  ).unwrap();

  let conn = state.db_pool.get().unwrap();
  let mut identity = match get_identity_db(conn, state.ipfs_id.clone()).await {
    Ok(res) => {
      println!("got : {:?}", res);
      res
    }
    Err(e) => {
      eprintln!("{:#?}", e);
      Identity::new(state.ipfs_id.clone())
    }
  };

  identity.posts.push(cid.clone());

  let conn = state.db_pool.get().unwrap();
  let identity = db_update_identity(conn, identity).await;
  println!("identity updated with: {:?}", identity);

  let post_response = PostResponse {
    post: post,
    cid: cid,
  };
  Ok(post_response)
}

pub async fn insert_post(conn: PooledConnection<SqliteConnectionManager>, post: Post) -> Post {
  conn.execute(
    "INSERT INTO posts (aux,body,files,files_cid,meta,publisher,ts) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    params![
      serde_json::to_value(&post.aux).unwrap(),
      &post.body,
      serde_json::to_value(&post.files).unwrap(),
      &post.files_cid,
      serde_json::to_value(&post.meta).unwrap(),
      &post.publisher,
      &post.ts,
    ],
  ).unwrap();
  post
}

#[tauri::command]
pub async fn initialize_database(publisher: String) -> Result<()> {
  println!("initialize_database: {:?}", publisher.clone());
  let mut conn = Connection::open(String::from(publisher.clone() + ".db"))?;
  let migrations = Migrations::new(vec![
    M::up(create_identities_table),
    M::up(create_posts_table),
    // In the future, add more migrations here
  ]);
  println!("running migrations...");
  migrations.to_latest(&mut conn).unwrap();

  let me = Identity::new(publisher.clone());
  match conn.execute(
      "INSERT INTO identities (aux,av,dn,following,meta,posts,publisher,ts) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
      params![
          // me.aux,
          json!(me.aux),
          me.av,
          me.dn,
          // me.following,
          json!(me.following),
          // me.meta,
          json!(me.meta),
          // me.posts,
          json!(me.posts),
          me.publisher,
          me.ts,
      ],
  ){
    Ok(i) => {
      println!("db initialized: {:?}", i);
      println!("closing conn: {:?}", conn.close());
    },
    Err(e) => {
      eprintln!("failed to initialize db: {:?}", e);
      println!("closing conn: {:?}", conn.close());
    }
  };

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

pub async fn wait_for_ipfs_id(client: &IpfsClient) -> Result<String, String> {
  let mut ready = false;
  let mut retries = 1;
  let mut identity = "".to_string();
  while !ready {
    match client.id(None).await {
      Ok(id) => {
        identity = id.id;
        ready = true;
      }
      Err(_err) => {
        if retries > 300 {
          break;
        }
        retries += 1;
        thread::sleep(Duration::from_millis(100));
      }
    }
  }

  Ok(identity)
}

pub async fn wait_for_ipfs_ready(client: &IpfsClient) -> Result<bool, bool> {
  let mut ready = false;
  let mut retries = 1;
  while !ready {
    match client.id(None).await {
      Ok(_id) => {
        ready = true;
      }
      Err(_err) => {
        if retries > 300 {
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
