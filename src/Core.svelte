<script context="module" lang="ts">
  import Database from "tauri-plugin-sql-api";
  import type { AddResult } from "ipfs-core-types/src/root";
  import type { Identity, IdentityResponse, Post } from "./types.type";
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
    publisher: string
  ): Promise<Identity> {
    console.log("getIdentityFromDB");
    const db = await Database.load("sqlite:sqlite.db");
    const rows: Identity[] = await db.select(
      "SELECT avatar,description,display_name,following,meta,posts,publisher,timestamp FROM identities WHERE publisher = ?",
      [publisher]
    );
    return rows[0];
  }

  export async function getIdentityFromIPFS(
    publisher: string
  ): Promise<Identity> {
    console.log("getIdentityFromIPFS");
    let bufs = [];
    const ipfs = await create({ url: "/ip4/127.0.0.1/tcp/5001" });
    let cid = await ipfs.resolve(publisher);
    if (!cid.includes("/identity.json")) {
      cid = cid + "/identity.json";
    }
    for await (const buf of ipfs.cat(cid)) {
      bufs.push(buf);
    }
    const buf: Buffer = Buffer.concat(bufs);
    return JSON.parse(buf.toString());
  }

  export async function getPostFromDB(cid: string): Promise<Post> {
    console.log("getPostFromDB");
    const db = await Database.load("sqlite:sqlite.db");
    const rows: Post[] = await db.select(
      "SELECT body,files,meta,publisher,timestamp FROM posts WHERE cid = ?",
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
    const ipfs = await create({ url: "/ip4/127.0.0.1/tcp/5001" });
    for await (const buf of ipfs.cat(cid)) {
      bufs.push(buf);
    }
    const buf: Buffer = Buffer.concat(bufs);
    return JSON.parse(buf.toString());
  }

  export async function followPublisher(publisher: string) {
    console.log("followPublisher");
    const ipfs = await create({ url: "/ip4/127.0.0.1/tcp/5001" });
    const ipfs_id = (await ipfs.id()).id;
    let identity: Identity = await getIdentityFromDB(ipfs_id);
    if (!identity.following.includes(publisher)) {
      identity.following.push(publisher);
      const identity_response = await publishIdentity(identity);
      console.log(identity_response);
      const update_result = await updateIdentityDB(identity_response);
      console.log(update_result);
    }
  }

  export async function deletePost(cid: string) {
    console.log("deletePost");
    const ipfs = await create({ url: "/ip4/127.0.0.1/tcp/5001" });
    const ipfs_id = (await ipfs.id()).id;
    let identity: Identity = await getIdentityFromDB(ipfs_id);
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

  export async function updateIdentity(identity: Identity) {
    console.log("updateIdentity");
    const ipfs = await create({ url: "/ip4/127.0.0.1/tcp/5001" });
    const ipfs_id = (await ipfs.id()).id;
    const db_identity: Identity = await getIdentityFromDB(ipfs_id);
    const updated_identity = { ...db_identity, ...identity };
    const identity_response = await publishIdentity(updated_identity);
    console.log(identity_response);
    const update_result = await updateIdentityDB(identity_response);
    console.log(update_result);
  }

  export async function publishIdentity(
    identity: Identity
  ): Promise<IdentityResponse> {
    console.log("publishIdentity");
    identity.timestamp = new Date().getTime();
    const json = JSON.stringify(identity);
    console.log(json);
    const ipfs = await create({ url: "/ip4/127.0.0.1/tcp/5001" });
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
    const identity_response: IdentityResponse = {
      cid: String(add_result.cid),
      identity: identity,
    };
    return identity_response;
  }

  export async function updateIdentityDB(ir: IdentityResponse) {
    console.log("getIdentityFromDB");
    const db = await Database.load("sqlite:sqlite.db");
    const result = await db.execute(
      "UPDATE identities SET cid=$1, avatar=$2, description=$3, display_name=$4, following=$5, meta=$6, posts=$7, publisher=$8, timestamp=$9 WHERE publisher=$10",
      [
        ir.cid,
        ir.identity.avatar,
        ir.identity.description,
        ir.identity.display_name,
        ir.identity.following,
        ir.identity.meta,
        ir.identity.posts,
        ir.identity.publisher,
        ir.identity.timestamp,
        ir.identity.publisher,
      ]
    );
    return result;
  }
</script>
