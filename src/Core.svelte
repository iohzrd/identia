<script context="module" lang="ts">
  import { create } from "ipfs-http-client/index";
  import Database from "tauri-plugin-sql-api";

  export async function followPublisher() {
    const db = await Database.load("sqlite:test.db");
  }
</script>
