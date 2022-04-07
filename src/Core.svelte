<script context="module" lang="ts">
  import Database from "tauri-plugin-sql-api";
  import type { AddResult } from "ipfs-core-types/src/root";
  import type { IPFSHTTPClient } from "ipfs-http-client";
  import type { Identity, Post } from "./types.type";
  import type { PublishResult } from "ipfs-core-types/src/name/index";
  import type { QueryResult } from "tauri-plugin-sql-api";
  import { Buffer } from "buffer/index";
  import { create } from "ipfs-http-client";

  export async function deleteIdentityFromDB(
    publisher: string
  ): Promise<QueryResult> {
    console.log("deleteIdentityFromDB: ", publisher);
    const db = await Database.load("sqlite:sqlite.db");
    return await db.execute("DELETE FROM identities WHERE publisher = ?", [
      publisher,
    ]);
  }

  export async function deletePostFromDB(cid: string): Promise<QueryResult> {
    console.log("deletePostFromDB: ", cid);
    const db = await Database.load("sqlite:sqlite.db");
    return await db.execute("DELETE FROM posts WHERE cid = ?", [cid]);
  }

  export async function getIdentityFromDB(
    publisher: string = undefined
  ): Promise<Identity> {
    console.log("getIdentityFromDB: ", publisher);
    if (publisher === undefined) {
      const ipfs: IPFSHTTPClient = await create({
        url: "/ip4/127.0.0.1/tcp/5001",
      });
      publisher = (await ipfs.id()).id;
    }
    const db = await Database.load("sqlite:sqlite.db");
    const rows: Identity[] = await db.select(
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
    const ipfs: IPFSHTTPClient = await create({
      url: "/ip4/127.0.0.1/tcp/5001",
    });
    let cid = await ipfs.resolve(publisher);
    let path;
    if (!cid.includes("/identity.json")) {
      path = cid + "/identity.json";
    }
    let bufs = [];
    for await (const buf of ipfs.cat(path)) {
      bufs.push(buf);
    }
    const buf: Buffer = Buffer.concat(bufs);
    return {
      cid: cid,
      ...JSON.parse(buf.toString()),
    };
  }

  export async function postInDB(cid: string): Promise<boolean> {
    console.log("postInDB: ", cid);
    const db = await Database.load("sqlite:sqlite.db");
    const rows: object[] = await db.select(
      "SELECT timestamp FROM posts WHERE cid = ?",
      [cid]
    );
    return rows.length > 0;
  }

  export async function getPostFromDB(cid: string): Promise<Post> {
    console.log("getPostFromDB: ", cid);
    const db = await Database.load("sqlite:sqlite.db");
    const rows: Post[] = await db.select(
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
    let bufs = [];
    const ipfs: IPFSHTTPClient = await create({
      url: "/ip4/127.0.0.1/tcp/5001",
    });
    for await (const buf of ipfs.cat(path)) {
      bufs.push(buf);
    }
    const buf: Buffer = Buffer.concat(bufs);
    return {
      cid: cid,
      ...JSON.parse(buf.toString()),
    };
  }

  export async function deletePost(cid: string) {
    console.log("deletePost: ", cid);
    let identity: Identity = await getIdentityFromDB();
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
    const db = await Database.load("sqlite:sqlite.db");
    // const { lastInsertId: id } = await db.execute('INSERT INTO todos (title) VALUES ($1)', [title]);
    return await db.execute(
      "INSERT INTO posts (cid,body,files,meta,publisher,timestamp) VALUES ($1,$2,$3,$4,$5,$6)",
      [
        post.cid,
        post.body,
        post.files,
        post.meta,
        post.publisher,
        post.timestamp,
      ]
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
    const db = await Database.load("sqlite:sqlite.db");
    // const { lastInsertId: id } = await db.execute('INSERT INTO todos (title) VALUES ($1)', [title]);
    return await db.execute(
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
    const db = await Database.load("sqlite:sqlite.db");
    const result = await db.execute(
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
    const db = await Database.load("sqlite:sqlite.db");
    const rows: object[] = await db.select(
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
    const ipfs: IPFSHTTPClient = await create({
      url: "/ip4/127.0.0.1/tcp/5001",
    });
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
    let identity: Identity = await getIdentityFromDB();
    if (!identity.following.includes(publisher)) {
      await getIdentity(publisher);
      identity.following.push(publisher);
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
        const fid_db: Identity = await getIdentityFromDB(publisher);
        if (fid_ipfs) {
          await updateIdentityDB(fid_ipfs);
        }
        fid_ipfs.posts.forEach(async (cid) => {
          if (!(await postInDB(cid))) {
            let post = await getPostFromIPFS(cid);
            if (publisher === post.publisher) {
              let result = await insertPostDB(post);
              console.log(result);
            }
          }
        });
      }
    });
  }
</script>
