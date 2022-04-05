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
    const db = await Database.load("sqlite:sqlite.db");
    return await db.execute("DELETE FROM identities WHERE publisher = ?", [
      publisher,
    ]);
  }

  export async function deletePostFromDB(cid: string): Promise<QueryResult> {
    console.log("deletePostFromDB");
    const db = await Database.load("sqlite:sqlite.db");
    return await db.execute("DELETE FROM posts WHERE cid = ?", [cid]);
  }

  export async function getIdentityFromDB(
    publisher: string = undefined
  ): Promise<Identity> {
    console.log("getIdentityFromDB");
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
    let name = publisher;
    if (!publisher.includes("ipns/")) {
      name = "/ipns/" + publisher;
    }
    console.log(`getIdentityFromIPFS: ${publisher}`);
    let bufs = [];
    const ipfs: IPFSHTTPClient = await create({
      url: "/ip4/127.0.0.1/tcp/5001",
    });
    let cid = await ipfs.resolve(name);
    if (!cid.includes("/identity.json")) {
      cid = cid + "/identity.json";
    }
    for await (const buf of ipfs.cat(cid)) {
      bufs.push(buf);
    }
    const buf: Buffer = Buffer.concat(bufs);
    return {
      cid: cid,
      ...JSON.parse(buf.toString()),
    };
  }

  export async function getPostFromDB(cid: string): Promise<Post> {
    console.log("getPostFromDB");
    const db = await Database.load("sqlite:sqlite.db");
    const rows: Post[] = await db.select(
      "SELECT cid,body,files,meta,publisher,timestamp FROM posts WHERE cid = ?",
      [cid]
    );
    return rows[0];
  }

  export async function getPostFromIPFS(cid: string): Promise<Post> {
    console.log("getPostFromIPFS");
    if (!cid.includes("/post.json")) {
      cid = cid + "/post.json";
    }
    let bufs = [];
    const ipfs: IPFSHTTPClient = await create({
      url: "/ip4/127.0.0.1/tcp/5001",
    });
    for await (const buf of ipfs.cat(cid)) {
      bufs.push(buf);
    }
    const buf: Buffer = Buffer.concat(bufs);
    return JSON.parse(buf.toString());
  }

  export async function followPublisher(publisher: string) {
    console.log("followPublisher");
    let identity: Identity = await getIdentityFromDB();
    if (!identity.following.includes(publisher)) {
      identity.following.push(publisher);
      const identity_response = await publishIdentity(identity);
      console.log(identity_response);
      const update_result = await updateIdentityDB(identity_response);
      console.log(update_result);
    }
  }

  export async function deletePost(cid: string) {
    console.log(`deletePost: ${cid}`);
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

  export async function addPostDB(post: Post): Promise<QueryResult> {
    console.log("addPostDB");
    console.log(post);
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
    console.log("addPost");
    await addPostDB(post);
    const db_identity: Identity = await getIdentityFromDB();
    db_identity.posts.unshift(post.cid);
    const identity_response = await publishIdentity(db_identity);
    console.log(identity_response);
    const update_result = await updateIdentityDB(identity_response);
    console.log(update_result);
  }

  export async function updateIdentity(identity: Identity) {
    console.log("updateIdentity");
    const db_identity: Identity = await getIdentityFromDB();
    const updated_identity = { ...db_identity, ...identity };
    const identity_response = await publishIdentity(updated_identity);
    console.log(identity_response);
    const update_result = await updateIdentityDB(identity_response);
    console.log(update_result);
  }

  export async function publishIdentity(identity: Identity): Promise<Identity> {
    console.log("publishIdentity");
    identity.timestamp = new Date().getTime();
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
      cid: String(add_result.cid),
      ...identity,
    };
  }

  export async function updateIdentityDB(i: Identity) {
    console.log("updateIdentityDB");
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

  export async function updateFeed() {
    console.log("updateFeed");
    const i: Identity = await getIdentityFromDB();
    i.following.forEach(async (publisher) => {
      const following_identity: Identity = await getIdentityFromIPFS(publisher);
      if (following_identity) {
        await updateIdentityDB(following_identity);
        following_identity.posts.forEach(async (cid) => {
          let post = await getPostFromIPFS(cid);
          let result = await addPostDB(post);
        });
      }
    });
  }
</script>
