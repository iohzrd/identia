<script lang="ts">
  import Database from "tauri-plugin-sql-api";
  import MediaModal from "./MediaModal.svelte";
  import NewPost from "./NewPost.svelte";
  import Post from "./Post.svelte";
  import type { PostResponse } from "../types.type";
  import { create } from "ipfs-http-client/index";
  import { inview } from "svelte-inview/dist/";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  export let params = {};
  $: publisher = params["publisher"];

  let db;
  let ipfs;
  let ipfs_info;
  let ipfs_id: string;
  let update_feed_interval = null;
  let feed: PostResponse[] = [];
  let newest_ts: number = Math.floor(new Date().getTime());
  let oldest_ts: number = Math.floor(new Date().getTime());
  let limit: number = 10;
  $: feed_query = `SELECT posts.cid, posts.body, posts.files, posts.meta, posts.publisher, posts.timestamp, identities.display_name FROM posts INNER JOIN identities ON identities.publisher = posts.publisher WHERE posts.timestamp < ${oldest_ts} ORDER BY posts.timestamp DESC LIMIT ${limit}`;
  $: new_posts_query = `SELECT posts.cid, posts.body, posts.files, posts.meta, posts.publisher, posts.timestamp, identities.display_name FROM posts INNER JOIN identities ON identities.publisher = posts.publisher WHERE posts.publisher != '${publisher}' AND posts.timestamp > ${newest_ts} ORDER BY posts.timestamp DESC`;

  let media_modal_idx = 0;
  let media_modal_media = [];
  let media_modal_open = false;

  async function getFeedPage() {
    console.log(`getFeedPage: ${publisher}`);
    if (feed.length > 0) {
      newest_ts = feed[0].timestamp;
      oldest_ts = feed[feed.length - 1].timestamp;
    }
    let page: PostResponse[] = await db.select(feed_query);
    console.log("page");
    console.log(page);
    if (page.length > 0) {
      feed = [...feed, ...page];
      if (feed.length > 0) {
        newest_ts = feed[0].timestamp;
        oldest_ts = feed[feed.length - 1].timestamp;
      }
    }
  }

  function onPost(post: PostResponse) {
    // feed.unshift(post);
    // feed = feed;
    feed = [post, ...feed];
    newest_ts = feed[0].timestamp;
    oldest_ts = feed[feed.length - 1].timestamp;
  }

  async function updateIdentities() {
    console.log(`updateIdentities: ${publisher}`);
    if (feed.length > 0) {
      newest_ts = feed[0].timestamp;
      oldest_ts = feed[feed.length - 1].timestamp;
    }
    let new_posts: PostResponse[] = await invoke("update_feed", {
      query: new_posts_query,
    });
    if (new_posts.length > 0) {
      feed = [...new_posts, ...feed];
      if (feed.length > 0) {
        newest_ts = feed[0].timestamp;
        oldest_ts = feed[feed.length - 1].timestamp;
      }
    }
  }

  onMount(async () => {
    db = await Database.load(`sqlite:sqlite.db`);
    ipfs = await create("/ip4/127.0.0.1/tcp/5001");
    ipfs_info = await ipfs.id();
    ipfs_id = ipfs_info.id;
    getFeedPage();
    // update_feed_interval = setInterval(updateIdentities, 60 * 1000);
  });

  onDestroy(() => {
    // clearInterval(update_feed_interval);
  });
</script>

<MediaModal bind:media_modal_idx bind:media_modal_media bind:media_modal_open />

<NewPost {onPost} />

{#each feed as post_response}
  <Post
    {ipfs_id}
    cid={null}
    {post_response}
    bind:media_modal_idx
    bind:media_modal_media
    bind:media_modal_open
  />
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
