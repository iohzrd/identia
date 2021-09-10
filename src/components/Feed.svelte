<script lang="ts">
  import { Tile } from "carbon-components-svelte";
  import { inview } from "svelte-inview/dist/";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  import NewPost from "./NewPost.svelte";
  import Post from "./Post.svelte";
  import type { PostResponse } from "../types.type";
  export let ipfs_id: string;

  let update_feed_interval = null;
  let feed: PostResponse[] = [];
  let newest_ts: number = Math.floor(new Date().getTime());
  let oldest_ts: number = Math.floor(new Date().getTime());
  let limit: number = 10;
  $: feed_query = `SELECT cid,body,files,meta,publisher,ts FROM posts WHERE ts < ${oldest_ts} ORDER BY ts DESC LIMIT ${limit}`;
  $: new_posts_query = `SELECT cid,body,files,meta,publisher,ts FROM posts WHERE publisher != '${ipfs_id}' AND ts > ${newest_ts} ORDER BY ts DESC`;

  async function getFeedPage() {
    console.log(`getFeedPage: ${ipfs_id}`);
    if (feed.length > 0) {
      newest_ts = feed[0].post.ts;
      oldest_ts = feed[feed.length - 1].post.ts;
    }
    let page: PostResponse[] = await invoke("query_posts", {
      query: feed_query,
    });
    if (page.length > 0) {
      feed = [...feed, ...page];
      newest_ts = feed[0].post.ts;
      oldest_ts = feed[feed.length - 1].post.ts;
    }
  }

  function onPost(post: PostResponse) {
    feed = [post, ...feed];
  }

  async function updateIdentities() {
    console.log(`updateIdentities: ${ipfs_id}`);
    if (feed.length > 0) {
      newest_ts = feed[0].post.ts;
      oldest_ts = feed[feed.length - 1].post.ts;
    }
    let new_posts: PostResponse[] = await invoke("update_feed", {
      query: new_posts_query,
    });
    if (new_posts.length > 0) {
      feed = [...new_posts, ...feed];
      newest_ts = feed[0].post.ts;
      oldest_ts = feed[feed.length - 1].post.ts;
    }
  }

  onMount(async () => {
    getFeedPage();
    update_feed_interval = setInterval(updateIdentities, 60 * 1000);
  });

  onDestroy(() => {
    clearInterval(update_feed_interval);
  });
</script>

<div>
  <Tile>
    <NewPost {onPost} />

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
