<script context="module" lang="ts">
  import { create } from "ipfs-http-client/index";
  import Database from "tauri-plugin-sql-api";

  export async function followPublisher(publisher: string) {
    const db = await Database.load("sqlite:sqlite.db");
    const ipfs = await create("/ip4/127.0.0.1/tcp/5001");
    console.log("followPublisher");
    follow_waiting = true;
    let follow_success = await invoke("follow_publisher", {
      publisher: publisher_to_follow.trim(),
    });
    if (follow_success) {
      clearFollowModal();
    }
  }

  export async function getIdentity(publisher: string) {
    const db = await Database.load("sqlite:sqlite.db");
    const ipfs = await create("/ip4/127.0.0.1/tcp/5001");

    const select = await db.select(
      "SELECT cid,avatar,description,display_name,following,meta,posts,publisher,timestamp FROM identities WHERE publisher = ?",
      [publisher]
    );
    console.log(select);
  }

  // const insert = await db.execute("INSERT INTO identities (cid) VALUES (?1)", [
  //   "test",
  // ]);
</script>
