use chrono::{offset::Utc, DateTime};
use common_multipart_rfc7578::client::multipart::Form;
use futures::TryStreamExt;
use infer;
use ipfs_api::IpfsClient;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{named_params, params, Connection, Result};
use rusqlite_migration::{Migrations, M};
use serde_json::{from_slice, json, Value};
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::{thread, time::Duration};
use tauri;
use tauri::api::process::Command;

pub mod migrations;
pub mod types;
use crate::config;
use crate::identity::migrations::{CREATE_IDENTITIES_TABLE, CREATE_POSTS_TABLE};
use crate::identity::types::{
  AppState, Identity, IdentityResponse, MediaResponse, MimeRequest, Post, PostRequest, PostResponse,
};

#[tauri::command]
pub async fn ipfs_id(state: tauri::State<'_, AppState>) -> Result<String, String> {
  Ok(state.ipfs_id.clone())
}

#[tauri::command]
pub async fn wait_for_ipfs_id_cmd(state: tauri::State<'_, AppState>) -> Result<String, String> {
  match wait_for_ipfs_id(&state.ipfs_client).await {
    Ok(id) => Ok(id.clone()),
    Err(_) => Err(String::from("")),
  }
}

pub async fn identity_in_db(
  conn: PooledConnection<SqliteConnectionManager>,
  publisher: String,
) -> Result<bool, bool> {
  let mut stmt = conn
    .prepare("SELECT publisher FROM identities WHERE publisher = ?")
    .unwrap();
  Ok(stmt.exists(params![&publisher]).unwrap())
}

pub async fn post_in_db(
  conn: PooledConnection<SqliteConnectionManager>,
  cid: String,
) -> Result<bool, bool> {
  let mut stmt = conn.prepare("SELECT cid FROM posts WHERE cid = ?").unwrap();
  Ok(stmt.exists(params![&cid]).unwrap())
}

pub async fn insert_identity(
  conn: PooledConnection<SqliteConnectionManager>,
  identity: Identity,
) -> Identity {
  println!("insert_identity: {:?}", identity);
  conn.execute(
    "INSERT INTO identities (cid,avatar,description,display_name,following,meta,posts,publisher,timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
    params![
      String::from(""),
      &identity.avatar,
      &identity.description,
      &identity.display_name,
      serde_json::to_value(&identity.following).unwrap(),
      serde_json::to_value(&identity.meta).unwrap(),
      serde_json::to_value(&identity.posts).unwrap(),
      &identity.publisher,
      &identity.timestamp,
    ],
  ).unwrap();
  identity
}

pub async fn update_identity_db(
  conn: PooledConnection<SqliteConnectionManager>,
  identity_res: &IdentityResponse,
) {
  let stmt = conn.prepare(
    "UPDATE identities SET cid=:cid,avatar=:avatar,description=:description,display_name=:display_name,following=:following,meta=:meta,posts=:posts,publisher=:publisher,timestamp=:timestamp WHERE publisher=:publisher",
  );
  let mut s = match stmt {
    Ok(stmt) => {
      println!("stmt valid");
      stmt
    }
    Err(error) => {
      panic!("invalid sql in update_identity_db: {:?}", error)
    }
  };

  match s.execute(named_params! {
  ":cid": &identity_res.cid,
  ":avatar": &identity_res.identity.avatar,
  ":description": &identity_res.identity.description,
  ":display_name": &identity_res.identity.display_name,
  ":following": serde_json::to_value(&identity_res.identity.following).unwrap(),
  ":meta": serde_json::to_value(&identity_res.identity.meta).unwrap(),
  ":posts": serde_json::to_value(&identity_res.identity.posts).unwrap(),
  ":publisher": &identity_res.identity.publisher,
  ":timestamp": &identity_res.identity.timestamp})
  {
    Ok(i) => {
      println!("execute success: {:?}", i);
    }
    Err(e) => {
      eprintln!("execute failed: {:?}", e);
    }
  };
}

pub async fn get_identity_db(
  conn: PooledConnection<SqliteConnectionManager>,
  publisher: String,
) -> Result<IdentityResponse, IdentityResponse> {
  println!("get_identity_db: {:?}", publisher.clone());
  let stmt = conn
    .prepare("SELECT cid,avatar,description,display_name,following,meta,posts,publisher,timestamp FROM identities WHERE publisher = ?");
  let mut s = match stmt {
    Ok(stmt) => stmt,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  };
  let identity_res = s
    .query_row(params![&publisher], |row| {
      Ok(IdentityResponse {
        cid: row.get(0)?,
        identity: Identity {
          avatar: row.get(1)?,
          description: row.get(2)?,
          display_name: row.get(3)?,
          following: serde_json::from_value(row.get(4)?).unwrap(),
          meta: serde_json::from_value(row.get(5)?).unwrap(),
          posts: serde_json::from_value(row.get(6)?).unwrap(),
          publisher: row.get(7)?,
          timestamp: row.get(8)?,
        },
      })
    })
    .unwrap_or(IdentityResponse {
      cid: String::from(""),
      identity: Identity::new(publisher.clone(), 0),
    });

  println!("get_identity_db: {:?}", identity_res);
  Ok(identity_res)
}

#[tauri::command]
pub async fn get_display_name_db(
  state: tauri::State<'_, AppState>,
  publisher: String,
) -> Result<String, String> {
  let conn = state.db_pool.get().unwrap();
  let stmt = conn.prepare("SELECT display_name FROM identities WHERE publisher = ?");
  let mut s = match stmt {
    Ok(stmt) => stmt,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    }
  };
  let display_name = s
    .query_row(params![&publisher], |row| Ok(row.get(0)?))
    .unwrap_or(String::from(""));
  Ok(display_name)
}

#[tauri::command]
pub async fn get_identity(
  state: tauri::State<'_, AppState>,
  publisher: String,
) -> Result<IdentityResponse, IdentityResponse> {
  let conn = state.db_pool.get().unwrap();
  let result = get_identity_db(conn, publisher.clone()).await;
  let identity_res = match result {
    Ok(i) => i,
    Err(_) => {
      let conn2 = state.db_pool.get().unwrap();
      IdentityResponse {
        cid: String::from(""),
        identity: insert_identity(conn2, Identity::new(publisher.clone(), 0)).await,
      }
    }
  };
  Ok(identity_res)
}

#[tauri::command]
pub async fn get_identity_ipfs_cmd(
  state: tauri::State<'_, AppState>,
  publisher: String,
) -> Result<IdentityResponse, IdentityResponse> {
  match get_identity_ipfs(&state.ipfs_client, publisher.clone()).await {
    Ok(identity_res) => Ok(identity_res),
    Err(_) => Err(IdentityResponse {
      cid: String::from(""),
      identity: Identity::new(publisher.clone(), 0),
    }),
  }
}

#[tauri::command]
pub async fn pin_cid(state: tauri::State<'_, AppState>, cid: String) -> Result<bool, bool> {
  println!("pin_cid: {:?}", cid);
  match state.ipfs_client.pin_add(cid.as_str(), true).await {
    Ok(res) => Ok({
      println!("pinAddResponse: {:?}", res);
      true
    }),
    Err(e) => Err({
      eprintln!("pinAddResponseError: {:?}", e);
      false
    }),
  }
}

#[tauri::command]
pub async fn get_file_ipfs(
  state: tauri::State<'_, AppState>,
  cid: String,
) -> Result<MediaResponse, MediaResponse> {
  println!("get_file_ipfs: {:?}", cid);
  let buf = state
    .ipfs_client
    .cat(cid.as_str())
    .map_ok(|chunk| chunk.to_vec())
    .try_concat()
    .await;

  match buf {
    Ok(buf) => Ok({
      let kind = infer::get(&buf).expect("file type is known");
      MediaResponse {
        data: buf,
        ext: String::from(kind.extension()),
        mime: String::from(kind.mime_type()),
      }
    }),
    Err(err) => Err({
      eprintln!("error getting file from ipfs: {:?}", err);
      MediaResponse {
        data: vec![],
        ext: String::from(""),
        mime: String::from(""),
      }
    }),
  }
}

#[tauri::command]
pub async fn get_mime(buf: MimeRequest) -> Result<String, String> {
  println!("get_mime: {:?}", &buf.data);
  let kind = infer::get(&buf.data).expect("failed to get mime");
  Ok(kind.mime_type().into())
}

#[tauri::command]
pub async fn update_feed(
  state: tauri::State<'_, AppState>,
  query: String,
) -> Result<Vec<PostResponse>, Vec<PostResponse>> {
  println!("update_feed");
  let conn = state.db_pool.get().unwrap();
  let identity_res = get_identity_db(conn, state.ipfs_id.clone()).await.unwrap();
  let following = identity_res.identity.following;
  let ipfs_id = state.ipfs_id.clone();
  for fid in following {
    if !ipfs_id.eq(&fid) {
      let f_identity = get_identity_ipfs(&state.ipfs_client, fid.clone())
        .await
        .unwrap();
      println!("got identity from ipfs");
      let conn = state.db_pool.get().unwrap();
      let db_identity_res = get_identity_db(conn, fid.clone()).await.unwrap();
      let posts = f_identity.identity.posts.clone();
      if f_identity.identity.timestamp > db_identity_res.identity.timestamp {
        let conn = state.db_pool.get().unwrap();
        let in_db = identity_in_db(conn, fid.clone()).await.unwrap();
        if !in_db {
          let conn = state.db_pool.get().unwrap();
          insert_identity(conn, db_identity_res.identity).await;
        }

        let conn = state.db_pool.get().unwrap();
        println!("identity is new than one in db...");
        println!("inserting new identity: {:?}", f_identity);
        update_identity_db(conn, &f_identity).await;
        for post_cid in posts {
          let _ = pin_cid(state.clone(), post_cid.clone()).await;
          println!("attempting to get new post: {:?}", post_cid.clone());
          match get_post_ipfs(state.clone(), post_cid.clone()).await {
            Ok(post_response) => {
              let conn = state.db_pool.get().unwrap();
              let post_in_db = post_in_db(conn, post_response.cid.clone()).await.unwrap();
              if !post_in_db {
                let conn = state.db_pool.get().unwrap();
                insert_post(conn, post_response.post, post_response.cid.clone()).await;
              }
            }
            Err(err) => {
              panic!("get_post_ipfs: {:?}", err);
            }
          };
        }
      }
    }
  }

  // REFACTOR ME
  let mut new_posts: Vec<PostResponse> = Vec::new();
  let conn = state.db_pool.get().unwrap();
  println!("{:?}", query);
  // SELECT cid,body,files,meta,publisher,timestamp FROM posts WHERE...
  let stmt = conn.prepare(&query.as_str());
  let mut s = match stmt {
    Ok(stmt) => {
      println!("query valid");
      stmt
    }
    Err(error) => {
      panic!("query invalid: {:?}", error)
    }
  };

  let pr_iter = s
    .query_map([], |row| {
      let pr = PostResponse {
        cid: row.get(0)?,
        post: Post {
          body: row.get(1)?,
          files: serde_json::from_value(row.get(2)?).unwrap(),
          meta: serde_json::from_value(row.get(3)?).unwrap(),
          publisher: row.get(4)?,
          timestamp: row.get(5)?,
        },
        display_name: row.get(6)?,
      };
      Ok(pr)
    })
    .unwrap();

  for pr in pr_iter {
    new_posts.push(pr.unwrap());
  }

  Ok(new_posts)
}

// #[tauri::command]
// pub async fn publish_identity_cmd(
//   state: tauri::State<'_, AppState>,
//   mut identity: Identity,
// ) -> Result<bool, bool> {
//   let mut success = false;
//   let ipfs_id = state.ipfs_id.clone();
//   if ipfs_id.eq(&identity.publisher.clone()) {
//     identity.timestamp = DateTime::timestamp_millis(&Utc::now());

//     println!("publisher added, updating identity: {:?}", identity);
//     let conn = state.db_pool.get().unwrap();

//     let identity = identity;
//     let identity_res = publish_identity(identity).await.unwrap();
//     update_identity_db(conn, &identity_res).await;

//     success = true;
//   } else {
//     println!("cannot publish non-self identity: {:?}", identity);
//   }

//   Ok(success)
// }

#[tauri::command]
pub async fn follow_publisher(
  state: tauri::State<'_, AppState>,
  publisher: String,
) -> Result<bool, bool> {
  let mut success = false;
  let conn = state.db_pool.get().unwrap();
  let mut db_identity_res = get_identity_db(conn, state.ipfs_id.clone()).await.unwrap();
  let ipfs_id = state.ipfs_id.clone();
  if !db_identity_res.identity.following.contains(&publisher) && !ipfs_id.eq(&publisher) {
    db_identity_res.identity.following.push(publisher.clone());
    db_identity_res.identity.timestamp = DateTime::timestamp_millis(&Utc::now());

    println!(
      "publisher added, updating identity: {:?}",
      db_identity_res.identity
    );
    let conn = state.db_pool.get().unwrap();

    let identity_res = publish_identity(db_identity_res.identity).await.unwrap();
    update_identity_db(conn, &identity_res).await;

    success = true;
  } else {
    println!(
      "already following publisher {:?}, skipping... {:?}",
      publisher, db_identity_res.identity
    );
  }

  Ok(success)
}

#[tauri::command]
pub async fn update_identity_aux(
  state: tauri::State<'_, AppState>,
  desc: String,
  dn: String,
  meta: Value,
) -> Result<IdentityResponse, IdentityResponse> {
  let conn = state.db_pool.get().unwrap();
  let mut db_identity_res = get_identity_db(conn, state.ipfs_id.clone()).await.unwrap();
  db_identity_res.identity.description = desc;
  db_identity_res.identity.display_name = dn;
  db_identity_res.identity.meta = meta;
  db_identity_res.identity.timestamp = DateTime::timestamp_millis(&Utc::now());
  println!("updating identity meta: {:?}", db_identity_res.identity);
  let conn = state.db_pool.get().unwrap();
  let identity_res = publish_identity(db_identity_res.identity).await.unwrap();
  update_identity_db(conn, &identity_res).await;
  Ok(identity_res)
}

pub async fn get_identity_ipfs(
  client: &IpfsClient,
  publisher: String,
) -> Result<IdentityResponse, IdentityResponse> {
  println!("get_identity_ipfs");
  match client.name_resolve(Some(&publisher), false, true).await {
    Ok(res) => {
      let identity_cid = res.path;
      println!("identity_cid: {:#?}", identity_cid);

      let mut identity_dot_json: String = identity_cid.clone();
      if !identity_dot_json.contains("/identity.json") {
        identity_dot_json.push_str("/identity.json");
      }

      match client
        .cat(&identity_dot_json)
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
      {
        Ok(res) => {
          let identity: Identity = from_slice(&res).unwrap();
          println!("identity: {:#?}", identity);
          Ok(IdentityResponse {
            cid: identity_dot_json,
            identity: identity,
          })
        }
        Err(blank_identity) => {
          eprintln!("{:#?}", blank_identity);
          Ok(IdentityResponse {
            cid: String::from(""),
            identity: Identity::new(publisher.clone(), 0),
          })
        }
      }
    }
    Err(e) => {
      eprintln!("{:#?}", e);
      Ok(IdentityResponse {
        cid: String::from(""),
        identity: Identity::new(publisher.clone(), 0),
      })
    }
  }
}

pub async fn publish_identity(identity: Identity) -> Result<IdentityResponse, IdentityResponse> {
  let ipfs_client = IpfsClient::default();

  let add = ipfs_api::request::Add::builder()
    .wrap_with_directory(true)
    .build();
  let mut form = Form::default();
  let json = serde_json::to_vec(&identity).unwrap();
  form.add_reader_file("path", Cursor::new(json), "identity.json");
  let cid = match ipfs_client.add_with_form(add, form).await {
    Ok(res) => {
      println!("res: {:?}", res);
      let mut cid = String::from("");
      for add in res {
        if add.name == String::from("") {
          cid = add.hash
        }
      }
      cid
    }
    Err(e) => {
      eprintln!("{:#?}", e);
      String::from("")
    }
  };

  match ipfs_client
    .name_publish(cid.as_str(), true, Some("24h"), None, None)
    .await
  {
    Ok(_) => {
      println!("publish complete");
    }
    Err(e) => {
      eprintln!("publish failed: {:#?}", e);
    }
  }
  Ok(IdentityResponse {
    cid: cid,
    identity: identity,
  })
}

#[tauri::command]
pub async fn get_post_ipfs(
  state: tauri::State<'_, AppState>,
  cid: String,
) -> Result<PostResponse, PostResponse> {
  let mut post_dot_json: String = cid.clone();
  if !post_dot_json.contains("/post.json") {
    post_dot_json.push_str("/post.json");
  }
  let client = IpfsClient::default();

  match client
    .cat(&post_dot_json)
    .map_ok(|chunk| chunk.to_vec())
    .try_concat()
    .await
  {
    Ok(res) => {
      let post: Post = from_slice(&res).unwrap();
      let display_name = get_display_name_db(state.clone(), post.publisher.clone())
        .await
        .unwrap();
      let post_response = PostResponse {
        cid: cid.clone(),
        post: post,
        display_name: display_name,
      };
      println!("{:#?}", post_response);
      Ok(post_response)
    }
    Err(e) => {
      eprintln!("{:#?}", e);
      let post_response = PostResponse {
        cid: String::from(""),
        post: Post {
          body: String::from(""),
          files: Vec::new(),
          meta: serde_json::to_value("{}").unwrap(),
          publisher: String::from(""),
          timestamp: 0,
        },
        display_name: String::from(""),
      };
      Ok(post_response)
    }
  }
}

// #[tauri::command]
// pub async fn get_post_ipfs(state: tauri::State<'_, AppState>, cid: String) -> Result<PostResponse> {
//   let mut post_dot_json: String = cid.clone();
//   if !post_dot_json.contains("/post.json") {
//     post_dot_json.push_str("/post.json");
//   }
//   let client = IpfsClient::default();
//   let post_response = match client
//     .cat(&post_dot_json)
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
pub async fn query_posts(
  state: tauri::State<'_, AppState>,
  query: String,
) -> Result<Vec<PostResponse>, Vec<PostResponse>> {
  let mut feed: Vec<PostResponse> = Vec::new();
  let conn = state.db_pool.get().unwrap();
  println!("{:?}", query);
  // SELECT cid,body,files,meta,publisher,timestamp FROM posts WHERE...
  let stmt = conn.prepare(&query.as_str());
  let mut s = match stmt {
    Ok(stmt) => {
      println!("query valid");
      stmt
    }
    Err(error) => {
      panic!("query invalid: {:?}", error)
    }
  };

  let pr_iter = s
    .query_map([], |row| {
      let pr = PostResponse {
        cid: row.get(0)?,
        post: Post {
          body: row.get(1)?,
          files: serde_json::from_value(row.get(2)?).unwrap(),
          meta: serde_json::from_value(row.get(3)?).unwrap(),
          publisher: row.get(4)?,
          timestamp: row.get(5)?,
        },
        display_name: row.get(6)?,
      };
      Ok(pr)
    })
    .unwrap();

  for pr in pr_iter {
    feed.push(pr.unwrap());
  }

  Ok(feed)
}

#[tauri::command]
pub async fn post(
  state: tauri::State<'_, AppState>,
  post_request: PostRequest,
) -> Result<PostResponse, PostResponse> {
  println!("post");
  println!("{:?}", post_request);
  let file_paths: Vec<String> = post_request.files.clone();
  let mut file_names: Vec<String> = Vec::new();

  let add = ipfs_api::request::Add::builder()
    .wrap_with_directory(true)
    .build();
  let mut form = Form::default();

  for filepath in file_paths {
    let data = fs::read(filepath.clone()).expect("Something went wrong reading the file");
    let filename = String::from(Path::new(&filepath).file_name().unwrap().to_str().unwrap());
    file_names.push(filename.clone());
    form.add_reader_file("path", Cursor::new(data), filename);
  }

  let post = Post::new(
    post_request.body,
    file_names,
    post_request.meta,
    state.ipfs_id.clone(),
  );
  println!("{:?}", post);
  let json = serde_json::to_vec(&post).unwrap();
  form.add_reader_file("path", Cursor::new(json), "post.json");

  let cid = match state.ipfs_client.add_with_form(add, form).await {
    Ok(res) => {
      println!("res: {:?}", res);
      let mut cid = String::from("");
      for add in res {
        if add.name == String::from("") {
          cid = add.hash
        }
      }
      cid
    }
    Err(e) => {
      eprintln!("{:#?}", e);
      String::from("")
    }
  };

  let conn = state.db_pool.get().unwrap();
  conn
    .execute(
      "INSERT INTO posts (cid,body,files,meta,publisher,timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
      params![
        &cid,
        &post.body,
        serde_json::to_value(&post.files).unwrap(),
        serde_json::to_value(&post.meta).unwrap(),
        &post.publisher,
        &post.timestamp,
      ],
    )
    .unwrap();

  let conn = state.db_pool.get().unwrap();
  let mut identity_res = match get_identity_db(conn, state.ipfs_id.clone()).await {
    Ok(res) => {
      println!("got : {:?}", res);
      res
    }
    Err(e) => {
      eprintln!("{:#?}", e);
      IdentityResponse {
        cid: String::from(""),
        identity: Identity::new(state.ipfs_id.clone(), 0),
      }
    }
  };

  identity_res.identity.posts.insert(0, cid.clone());
  identity_res.identity.timestamp = DateTime::timestamp_millis(&Utc::now());

  let conn = state.db_pool.get().unwrap();
  println!("identity updated with: {:?}", identity_res.identity);
  println!("publishing identity...");
  let identity_res = publish_identity(identity_res.identity).await.unwrap();
  update_identity_db(conn, &identity_res).await;

  let display_name = 0;
  let post_response = PostResponse {
    cid: cid,
    post: post,
    display_name: identity_res.identity.display_name,
  };
  Ok(post_response)
}

pub async fn insert_post(
  conn: PooledConnection<SqliteConnectionManager>,
  post: Post,
  cid: String,
) -> Post {
  conn
    .execute(
      "INSERT INTO posts (cid,body,files,meta,publisher,timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
      params![
        &cid,
        &post.body,
        serde_json::to_value(&post.files).unwrap(),
        serde_json::to_value(&post.meta).unwrap(),
        &post.publisher,
        &post.timestamp,
      ],
    )
    .unwrap();
  post
}

#[tauri::command]
pub async fn initialize_database(publisher: String, db_file_path: PathBuf) -> Result<()> {
  println!("initialize_database: {:?}", publisher.clone());
  let mut conn = Connection::open(db_file_path.into_os_string().to_str().unwrap())?;
  let migrations = Migrations::new(vec![
    M::up(CREATE_IDENTITIES_TABLE),
    M::up(CREATE_POSTS_TABLE),
    // In the future, add more migrations here
  ]);
  println!("running migrations...");
  migrations.to_latest(&mut conn).unwrap();

  let me = Identity::new(publisher.clone(), DateTime::timestamp_millis(&Utc::now()));
  match conn.execute(
      "INSERT INTO identities (cid,avatar,description,display_name,following,meta,posts,publisher,timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
      params![
          String::from(""),
          me.avatar,
          me.description,
          me.display_name,
          json!(me.following),
          json!(me.meta),
          json!(me.posts),
          me.publisher,
          me.timestamp,
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

pub fn initialize_ipfs() -> bool {
  config::create_initial_config_if_necessary();
  println!("Initializing IPFS");
  let cmd = Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "init",
      "-c",
      config::identia_app_data_path()
        .into_os_string()
        .to_str()
        .unwrap(),
    ])
    .output()
    .unwrap();
  format!("ipfs init: {:?}", cmd);
  true
}

pub fn initialize_ipfs_config() {
  println!("configuring IPFS");
  Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "-c",
      config::identia_app_data_path()
        .into_os_string()
        .to_str()
        .unwrap(),
      "config",
      "--json",
      "API.HTTPHeaders.Access-Control-Allow-Origin",
      r#"["tauri://localhost","https://tauri.localhost"]"#,
    ])
    .output()
    .unwrap();
  Command::new_sidecar("ipfs")
    .unwrap()
    .args(&[
      "-c",
      config::identia_app_data_path()
        .into_os_string()
        .to_str()
        .unwrap(),
      "config",
      "--json",
      "API.HTTPHeaders.Access-Control-Allow-Methods",
      r#"["GET","POST","PUT"]"#,
    ])
    .output()
    .unwrap();
}

pub async fn launch_ipfs_daemon(client: &IpfsClient) -> Result<String, String> {
  println!("Starting IPFS.");
  Command::new_sidecar("ipfs")
    .or(Err(String::from("Can't find ipfs binary")))?
    .args(&[
      "daemon",
      "-c",
      config::identia_app_data_path()
        .into_os_string()
        .to_str()
        .unwrap(),
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
        if retries > 600 {
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
