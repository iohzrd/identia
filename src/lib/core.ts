import all from "it-all";
import type { AddResult } from "ipfs-core-types/src/root";
import type { IPFSHTTPClient } from "ipfs-http-client";
import type { Identity, Post } from "./types";
import type { PublishResult } from "ipfs-core-types/src/name/index";
import type { QueryResult } from "tauri-plugin-sql-api";
import { concat, toString } from "uint8arrays";
import { create } from "ipfs-http-client";
import { execute, select } from "./db";

const DEBUG = import.meta.env.DEV;
export function log(message?: any, ...optionalParams: any[]) {
  if (DEBUG) {
    console.log(message, ...optionalParams);
  }
}

export const ipfs: IPFSHTTPClient = create({
  url: "/ip4/127.0.0.1/tcp/5001",
});

export async function deleteIdentityFromDB(
  publisher: string
): Promise<QueryResult> {
  return await execute("DELETE FROM identities WHERE publisher = ?", [
    publisher,
  ]);
}

export async function deletePostFromDB(cid: string): Promise<QueryResult> {
  return await execute("DELETE FROM posts WHERE cid = ?", [cid]);
}

export async function deletePublisherPostsFromDB(
  publisher: string
): Promise<QueryResult> {
  return await execute("DELETE FROM posts WHERE publisher = ?", [publisher]);
}

export async function getIdentityFromDB(
  publisher: string | undefined = undefined
): Promise<Identity> {
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
  if (!publisher.includes("/ipns/")) {
    publisher = "/ipns/" + publisher;
  }
  const cid = await ipfs.resolve(publisher);
  let path: string = cid;
  if (!path.includes("/identity.json")) {
    path = path + "/identity.json";
  }
  return {
    cid: cid,
    ...JSON.parse(toString(concat(await all(ipfs.cat(path))))),
  };
}

export async function postInDB(cid: string): Promise<boolean> {
  const rows: object[] = await select(
    "SELECT timestamp FROM posts WHERE cid = ?",
    [cid]
  );
  return rows.length > 0;
}

export async function getPostFromDB(cid: string): Promise<Post> {
  const rows: Post[] = await select(
    "SELECT cid,body,files,meta,publisher,timestamp FROM posts WHERE cid = ?",
    [cid]
  );
  return rows[0];
}

export async function getPostCidsFromDB(publisher: string): Promise<string[]> {
  const rows: Post[] = await select(
    "SELECT cid FROM posts WHERE publisher = ? ORDER BY timestamp DESC",
    [publisher]
  );
  return rows.map((e) => e.cid);
}

export async function getPostFromIPFS(cid: string): Promise<Post> {
  let path: string = cid;
  if (!path.includes("/post.json")) {
    path = path + "/post.json";
  }
  await ipfs.pin.add(cid);
  return {
    cid: cid,
    ...JSON.parse(toString(concat(await all(ipfs.cat(path))))),
  };
}

export async function deletePost(cid: string) {
  await deletePostFromDB(cid);
  const identity: Identity = await getIdentityFromDB();
  identity.posts = await getPostCidsFromDB(identity.publisher);
  const identity_response = await publishIdentity(identity);
  await updateIdentityDB(identity_response);
}

export async function insertPostDB(post: Post): Promise<QueryResult> {
  return await execute(
    "INSERT INTO posts (cid,body,files,meta,publisher,timestamp) VALUES ($1,$2,$3,$4,$5,$6)",
    [post.cid, post.body, post.files, post.meta, post.publisher, post.timestamp]
  );
}

export async function addPost(post: Post) {
  await insertPostDB(post);
  const db_identity: Identity = await getIdentityFromDB();
  // db_identity.posts.unshift(post.cid);
  db_identity.posts = await getPostCidsFromDB(db_identity.publisher);
  const identity_response = await publishIdentity(db_identity);
  await updateIdentityDB(identity_response);
}

export async function insertIdentityDB(
  identity: Identity
): Promise<QueryResult> {
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
  const db_identity: Identity = await getIdentityFromDB();
  const updated_identity = { ...db_identity, ...identity };
  const identity_response = await publishIdentity(updated_identity);
  await updateIdentityDB(identity_response);
}

export async function updateIdentityDB(i: Identity) {
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
  const rows: object[] = await select(
    "SELECT timestamp FROM identities WHERE publisher = ?",
    [publisher]
  );
  return rows.length > 0;
}

export async function publishIdentity(
  identity: Identity | undefined = undefined
): Promise<Identity> {
  if (identity == undefined) {
    identity = await getIdentityFromDB();
  }
  identity.timestamp = new Date().getTime();
  if (!identity.cid?.includes("/ipfs/")) {
    identity.cid = "/ipfs/" + identity.cid;
  }
  identity.provenance = identity.cid;
  delete identity.cid;
  const json = JSON.stringify(identity);
  const obj = {
    path: "identity.json",
    content: json,
  };
  const add_result: AddResult = await ipfs.add(obj, {
    wrapWithDirectory: true,
  });
  await ipfs.name.publish(String(add_result.cid));
  return {
    ...identity,
    cid: String(add_result.cid),
  };
}

export async function republishIdentity() {
  const identity = await getIdentityFromDB();
  const interval = new Date().getTime() - 1000 * 60 * 60 * 4;
  if (identity.timestamp < interval) {
    identity.timestamp = new Date().getTime();
    await ipfs.name.publish(String(identity.cid));
    await updateIdentityDB(identity);
    // const published = await publishIdentity();
    // await updateIdentityDB(published);
  }
}

export function getNewIdentity(publisher: string): Identity {
  return {
    cid: "",
    avatar: "",
    description: "",
    display_name: "",
    following: [],
    meta: {},
    posts: [],
    publisher: publisher,
    provenance: "",
    timestamp: 0,
  };
}

export async function getIdentity(publisher: string): Promise<Identity> {
  if (!(await identityInDB(publisher))) {
    await insertIdentityDB(getNewIdentity(publisher));
  }
  return await getIdentityFromDB(publisher);
}

export async function followPublisher(publisher: string) {
  const identity: Identity = await getIdentityFromDB();
  if (!identity.following.includes(publisher)) {
    await getIdentity(publisher);
    identity.following.push(publisher);
    const identity_response = await publishIdentity(identity);
    await updateIdentityDB(identity_response);
  }
}

export async function unfollowPublisher(publisher: string) {
  const identity: Identity = await getIdentityFromDB();
  if (
    publisher !== identity.publisher &&
    identity.following.includes(publisher)
  ) {
    await deletePublisherPostsFromDB(publisher);
    identity.following = identity.following.filter((p) => p !== publisher);
    const identity_response = await publishIdentity(identity);
    await updateIdentityDB(identity_response);
  }
}

export async function updateFeed() {
  const i: Identity = await getIdentityFromDB();
  i.following.forEach(async (publisher) => {
    if (publisher != i.publisher) {
      try {
        const fid_ipfs: Identity = await getIdentityFromIPFS(publisher);
        if (fid_ipfs) {
          const fid_db: Identity = await getIdentityFromDB(publisher);
          if (fid_db && fid_db.timestamp < fid_ipfs.timestamp) {
            await updateIdentityDB(fid_ipfs);
            fid_ipfs.posts.forEach(async (cid) => {
              if (!(await postInDB(cid))) {
                const post = await getPostFromIPFS(cid);
                if (publisher === post.publisher) {
                  await insertPostDB(post);
                }
              }
            });
          }
        }
      } catch (error) {}
    }
  });
}
