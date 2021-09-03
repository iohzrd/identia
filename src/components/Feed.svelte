<script lang="ts">
  import { Tile } from "carbon-components-svelte";
  import { emit, listen } from "@tauri-apps/api/event";
  import { inview } from "svelte-inview/dist/";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  import NewPost from "./NewPost.svelte";
  import Post from "./Post.svelte";
  import type { PostResponse } from "../types.type";
  export let ipfs_id: string;

  let feed: PostResponse[] = [];
  let oldest_ts: number = Math.floor(new Date().getTime());
  let limit: number = 10;
  $: query = `SELECT cid,aux,body,files,meta,publisher,ts FROM posts WHERE ts < ${oldest_ts} ORDER BY ts DESC LIMIT ${limit}`;

  async function getFeedPage() {
    console.log(`getFeedPage: ${ipfs_id}`);
    if (feed.length > 0) {
      oldest_ts = feed[feed.length - 1].post.ts;
    }
    let page: PostResponse[] = await invoke("query_posts", {
      query: query,
    });
    if (page.length > 0) {
      feed = [...feed, ...page];
      oldest_ts = feed[feed.length - 1].post.ts;
    }
  }

  onMount(async () => {
    getFeedPage();
  });

  onDestroy(() => {});
</script>

<div>
  <Tile>
    <NewPost />

    {#each feed as postResponse}
      <Post cid={null} {postResponse} />
    {/each}
  </Tile>

  {#if feed.length >= limit}
    <div
      use:inview={{}}
      on:enter={(event) => {
        if (event.detail.inView) {
          getFeedPage();
        }
      }}
    />
  {/if}
</div>
