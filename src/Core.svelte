<script context="module" lang="ts">
  import Database from "tauri-plugin-sql-api";
  import type { AddResult } from "ipfs-core-types/src/root";
  import type { Identity, IdentityResponse } from "./types.type";
  import type { PublishResult } from "ipfs-core-types/src/name/index";
  import { Buffer } from "buffer/index";
  import { create } from "ipfs-http-client";

  export async function followPublisher(publisher: string) {}

  export async function getPostFromIPFS(cid: string) {
    console.log("getPostIPFS");
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

  export async function getIdentityFromDB(publisher: string) {
    console.log("getIdentityFromDB");
    const db = await Database.load("sqlite:sqlite.db");
    const rows = await db.select(
      "SELECT * FROM identities WHERE publisher = ?",
      [publisher]
    );
    return rows[0];
  }

  // WIP
  export async function publishIdentity(
    identity: Identity
  ): Promise<IdentityResponse> {
    console.log("publishIdentity");
    identity.timestamp = Math.floor(new Date().getTime());
    const json = JSON.stringify(identity);
    const ipfs = await create({ url: "/ip4/127.0.0.1/tcp/5001" });
    const obj = {
      path: "identity.json",
      content: json,
    };
    const add_result: AddResult = await ipfs.add(obj, {
      wrapWithDirectory: true,
    });
    const publish_result: PublishResult = await ipfs.name.publish(
      add_result.cid.toString()
    );
    const identity_response: IdentityResponse = {
      cid: "",
      identity: identity,
    };
    console.log("publish complete");
    console.log(publish_result);
    return identity_response;
  }

  export async function updateIdentity(identity: Identity) {
    const ipfs = await create({ url: "/ip4/127.0.0.1/tcp/5001" });
    const ipfs_id = (await ipfs.id()).id;
    const db_identity: Identity = await getIdentityFromDB(ipfs_id);
    const updated_identity = { ...db_identity, ...identity };
    updated_identity.timestamp = Math.floor(new Date().getTime());
    const publish_result = await publishIdentity(updated_identity);
    updated_identity.cid = publish_result.value;
  }

  export async function updateIdentityInDB(i: Identity) {
    console.log("getIdentityFromDB");
    const db = await Database.load("sqlite:sqlite.db");
    const cid = "";
    const rows = await db.execute(
      `UPDATE identities SET 
        cid=$1,
        avatar=$2,
        description=$3,
        display_name=$4,
        following=$5,
        meta=$6,
        posts=$7,
        publisher=$8,
        timestamp=$9
        WHERE publisher=:publisher`,
      [
        cid,
        i.avatar,
        i.description,
        i.display_name,
        i.following,
        i.meta,
        i.posts,
        i.publisher,
        i.timestamp,
      ]
    );
    return rows[0];
  }
</script>
