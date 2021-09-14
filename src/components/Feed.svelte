<script lang="ts">
  import { inview } from "svelte-inview/dist/";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  import NewPost from "./NewPost.svelte";
  import Post from "./Post.svelte";
  import type { PostResponse } from "../types.type";

  export let params = {};
  $: publisher = params["publisher"];

  let update_feed_interval = null;
  let feed: PostResponse[] = [];
  let newest_ts: number = Math.floor(new Date().getTime());
  let oldest_ts: number = Math.floor(new Date().getTime());
  let limit: number = 10;
  $: feed_query = `SELECT cid,body,files,meta,publisher,timestamp FROM posts WHERE timestamp < ${oldest_ts} ORDER BY timestamp DESC LIMIT ${limit}`;
  $: new_posts_query = `SELECT cid,body,files,meta,publisher,timestamp FROM posts WHERE publisher != '${publisher}' AND timestamp > ${newest_ts} ORDER BY timestamp DESC`;

  async function getFeedPage() {
    console.log(`getFeedPage: ${publisher}`);
    if (feed.length > 0) {
      newest_ts = feed[0].post.timestamp;
      oldest_ts = feed[feed.length - 1].post.timestamp;
    }
    let page: PostResponse[] = await invoke("query_posts", {
      query: feed_query,
    });
    if (page.length > 0) {
      feed = [...feed, ...page];
      newest_ts = feed[0].post.timestamp;
      oldest_ts = feed[feed.length - 1].post.timestamp;
    }
  }

  function onPost(post: PostResponse) {
    feed = [post, ...feed];
  }

  async function updateIdentities() {
    console.log(`updateIdentities: ${publisher}`);
    if (feed.length > 0) {
      newest_ts = feed[0].post.timestamp;
      oldest_ts = feed[feed.length - 1].post.timestamp;
    }
    let new_posts: PostResponse[] = await invoke("update_feed", {
      query: new_posts_query,
    });
    if (new_posts.length > 0) {
      feed = [...new_posts, ...feed];
      newest_ts = feed[0].post.timestamp;
      oldest_ts = feed[feed.length - 1].post.timestamp;
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

<NewPost {onPost} />

{#each feed as postResponse}
  <Post cid={null} {postResponse} />
{/each}

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
