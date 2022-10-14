import Database from "tauri-plugin-sql-api";
import all from "it-all";
import type { AddResult } from "ipfs-core-types/src/root";
import type { IPFSHTTPClient } from "ipfs-http-client";
import type { Identity, Post } from "./types";
import type { PublishResult } from "ipfs-core-types/src/name/index";
import type { QueryResult } from "tauri-plugin-sql-api";
import { concat } from "uint8arrays/concat";
import { create } from "ipfs-http-client";
import { toString } from "uint8arrays/to-string";

let db = null;
const loadDB = Database.load("sqlite:sqlite.db").then((instance) => {
  db = instance;
  return db;
});

export const ipfs: IPFSHTTPClient = create({
  url: "/ip4/127.0.0.1/tcp/5001",
});

export async function execute(query: string, bindValues?: unknown[]) {
  console.log("execute");
  await loadDB;
  return await db.execute(query, bindValues ?? []);
}

export async function select(query: string, bindValues?: unknown[]) {
  console.log("select");
  await loadDB;
  return await db.select(query, bindValues ?? []);
}

export async function deleteIdentityFromDB(
  publisher: string
): Promise<QueryResult> {
  console.log("deleteIdentityFromDB: ", publisher);
  return await execute("DELETE FROM identities WHERE publisher = ?", [
    publisher,
  ]);
}

export async function deletePostFromDB(cid: string): Promise<QueryResult> {
  console.log("deletePostFromDB: ", cid);
  return await execute("DELETE FROM posts WHERE cid = ?", [cid]);
}

export async function deletePublisherPostsFromDB(
  publisher: string
): Promise<QueryResult> {
  console.log("deletePublisherPostsFromDB: ", publisher);
  return await execute("DELETE FROM posts WHERE publisher = ?", [publisher]);
}

export async function getIdentityFromDB(
  publisher: string = undefined
): Promise<Identity> {
  console.log("getIdentityFromDB: ", publisher);
  if (publisher === undefined) {
    publisher = (await ipfs.id()).id.toString();
  }
  const rows: Identity[] = await select(
    "SELECT cid,avatar,description,display_name,following,meta,posts,publisher,timestamp FROM identities WHERE publisher = ?",
    [publisher]
  );
  return rows[0];
}

export async function getIdentityFromIPFS(
  publisher: string
): Promise<Identity> {
  console.log("getIdentityFromIPFS: ", publisher);

  if (!publisher.includes("/ipns/")) {
    publisher = "/ipns/" + publisher;
  }
  const cid = await ipfs.resolve(publisher);
  let path;
  if (!cid.includes("/identity.json")) {
    path = cid + "/identity.json";
  }
  return {
    cid: cid,
    ...JSON.parse(toString(concat(await all(ipfs.cat(path))))),
  };
}

export async function postInDB(cid: string): Promise<boolean> {
  console.log("postInDB: ", cid);
  const rows: object[] = await select(
    "SELECT timestamp FROM posts WHERE cid = ?",
    [cid]
  );
  return rows.length > 0;
}

export async function getPostFromDB(cid: string): Promise<Post> {
  console.log("getPostFromDB: ", cid);
  const rows: Post[] = await select(
    "SELECT cid,body,files,meta,publisher,timestamp FROM posts WHERE cid = ?",
    [cid]
  );
  return rows[0];
}

export async function getPostFromIPFS(cid: string): Promise<Post> {
  console.log("getPostFromIPFS: ", cid);
  let path;
  if (!cid.includes("/post.json")) {
    path = cid + "/post.json";
  }
  await ipfs.pin.add(cid);
  return {
    cid: cid,
    ...JSON.parse(toString(concat(await all(ipfs.cat(path))))),
  };
}

export async function deletePost(cid: string) {
  console.log("deletePost: ", cid);
  const identity: Identity = await getIdentityFromDB();
  if (identity.posts.includes(cid)) {
    identity.posts = identity.posts.filter((p) => p !== cid);
    const identity_response = await publishIdentity(identity);
    console.log(identity_response);
    const update_result = await updateIdentityDB(identity_response);
    console.log(update_result);
    const delete_result = await deletePostFromDB(cid);
    console.log(delete_result);
  }
}

export async function insertPostDB(post: Post): Promise<QueryResult> {
  console.log("insertPostDB: ", post);
  return await execute(
    "INSERT INTO posts (cid,body,files,meta,publisher,timestamp) VALUES ($1,$2,$3,$4,$5,$6)",
    [post.cid, post.body, post.files, post.meta, post.publisher, post.timestamp]
  );
}

export async function addPost(post: Post) {
  console.log("addPost: ", post);
  await insertPostDB(post);
  const db_identity: Identity = await getIdentityFromDB();
  db_identity.posts.unshift(post.cid);
  const identity_response = await publishIdentity(db_identity);
  console.log(identity_response);
  const update_result = await updateIdentityDB(identity_response);
  console.log(update_result);
}

export async function insertIdentityDB(
  identity: Identity
): Promise<QueryResult> {
  console.log("insertIdentityDB: ", identity);
  return await execute(
    "INSERT INTO identities (cid,avatar,description,display_name,following,meta,posts,publisher,timestamp) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)",
    [
      identity.cid,
      identity.avatar,
      identity.description,
      identity.display_name,
      identity.following,
      identity.meta,
      identity.posts,
      identity.publisher,
      identity.timestamp,
    ]
  );
}

export async function updateIdentity(identity: Identity) {
  console.log("updateIdentity: ", identity);
  const db_identity: Identity = await getIdentityFromDB();
  const updated_identity = { ...db_identity, ...identity };
  const identity_response = await publishIdentity(updated_identity);
  console.log(identity_response);
  const update_result = await updateIdentityDB(identity_response);
  console.log(update_result);
}

export async function updateIdentityDB(i: Identity) {
  console.log("updateIdentityDB: ", i);
  const result = await execute(
    "UPDATE identities SET cid=$1, avatar=$2, description=$3, display_name=$4, following=$5, meta=$6, posts=$7, publisher=$8, timestamp=$9 WHERE publisher=$10",
    [
      i.cid,
      i.avatar,
      i.description,
      i.display_name,
      i.following,
      i.meta,
      i.posts,
      i.publisher,
      i.timestamp,
      i.publisher,
    ]
  );
  return result;
}

export async function identityInDB(publisher: string): Promise<boolean> {
  console.log("identityInDB: ", publisher);
  const rows: object[] = await select(
    "SELECT timestamp FROM identities WHERE publisher = ?",
    [publisher]
  );
  return rows.length > 0;
}

export async function publishIdentity(identity: Identity): Promise<Identity> {
  console.log("publishIdentity: ", identity);
  identity.timestamp = new Date().getTime();
  delete identity.cid;
  const json = JSON.stringify(identity);
  console.log(json);
  const obj = {
    path: "identity.json",
    content: json,
  };
  const add_result: AddResult = await ipfs.add(obj, {
    wrapWithDirectory: true,
  });
  console.log(add_result);
  const publish_result: PublishResult = await ipfs.name.publish(
    String(add_result.cid)
  );
  console.log("publish complete");
  console.log(publish_result);
  return {
    ...identity,
    cid: String(add_result.cid),
  };
}

export function getNewIdentity(publisher: string): Identity {
  console.log("getNewIdentity: ", publisher);
  return {
    cid: "",
    avatar: "",
    description: "",
    display_name: "",
    following: [],
    meta: {},
    posts: [],
    publisher: publisher,
    timestamp: 0,
  };
}

export async function getIdentity(publisher: string): Promise<Identity> {
  console.log("getIdentity: ", publisher);
  if (!(await identityInDB(publisher))) {
    await insertIdentityDB(getNewIdentity(publisher));
  }
  return await getIdentityFromDB(publisher);
}

export async function followPublisher(publisher: string) {
  console.log("followPublisher: ", publisher);
  const identity: Identity = await getIdentityFromDB();
  if (!identity.following.includes(publisher)) {
    await getIdentity(publisher);
    identity.following.push(publisher);
    const identity_response = await publishIdentity(identity);
    console.log(identity_response);
    const update_result = await updateIdentityDB(identity_response);
    console.log(update_result);
  }
}

export async function unfollowPublisher(publisher: string) {
  console.log("unfollowPublisher: ", publisher);
  const identity: Identity = await getIdentityFromDB();
  if (
    publisher !== identity.publisher &&
    identity.following.includes(publisher)
  ) {
    const delete_result = await deletePublisherPostsFromDB(publisher);
    console.log(delete_result);
    identity.following = identity.following.filter((p) => p !== publisher);
    const identity_response = await publishIdentity(identity);
    console.log(identity_response);
    const update_result = await updateIdentityDB(identity_response);
    console.log(update_result);
  }
}

export async function updateFeed() {
  console.log("updateFeed");
  const i: Identity = await getIdentityFromDB();
  i.following.forEach(async (publisher) => {
    if (publisher != i.publisher) {
      const fid_ipfs: Identity = await getIdentityFromIPFS(publisher);
      if (fid_ipfs) {
        const fid_db: Identity = await getIdentityFromDB(publisher);
        if (fid_db && fid_db.timestamp < fid_ipfs.timestamp) {
          await updateIdentityDB(fid_ipfs);
          fid_ipfs.posts.forEach(async (cid) => {
            if (!(await postInDB(cid))) {
              const post = await getPostFromIPFS(cid);
              if (publisher === post.publisher) {
                const result = await insertPostDB(post);
                console.log(result);
              }
            }
          });
        }
      }
    }
  });
}
