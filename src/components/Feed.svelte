<script lang="ts">
  import Database from "tauri-plugin-sql-api";
  import MediaModalComponent from "./MediaModal.svelte";
  import NewPostComponent from "./NewPost.svelte";
  import PostComponent from "./Post.svelte";
  import type { IDResult } from "ipfs-core-types/src/root";
  import type { IPFSHTTPClient } from "ipfs-http-client";
  import type { Post } from "../types.type";
  import { create } from "ipfs-http-client";
  import { updateFeed } from "../Core.svelte";
  import { inview } from "svelte-inview/dist/";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  export let params = {};
  $: publisher = params["publisher"];

  let db;
  let ipfs: IPFSHTTPClient;
  let ipfs_info: IDResult;
  let ipfs_id: string;
  let update_feed_interval = null;
  let feed: Post[] = [];
  let newest_ts: number = new Date().getTime();
  let oldest_ts: number = new Date().getTime();
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
    let page: Post[] = await db.select(feed_query);
    console.log("page");
    console.log(page);
    if (page.length > 0) {
      feed = [...feed, ...page];
      newest_ts = feed[0].timestamp;
      oldest_ts = feed[feed.length - 1].timestamp;
    }
  }

  function onPost(post: Post) {
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
    // let new_posts: Post[] = await invoke("update_feed", {
    //   query: new_posts_query,
    // });
    // let new_posts: Post[] = await updateFeed()
    await updateFeed();

    // if (new_posts.length > 0) {
    //   feed = [...new_posts, ...feed];
    //   newest_ts = feed[0].timestamp;
    //   oldest_ts = feed[feed.length - 1].timestamp;
    // }
  }

  onMount(async () => {
    db = await Database.load(`sqlite:sqlite.db`);
    if (ipfs === undefined) {
      ipfs = await create({ url: "/ip4/127.0.0.1/tcp/5001" });
    }
    ipfs_info = await ipfs.id();
    ipfs_id = ipfs_info.id;
    getFeedPage();
    await updateFeed();
    update_feed_interval = setInterval(updateIdentities, 60 * 1000);
  });

  onDestroy(() => {
    clearInterval(update_feed_interval);
  });
</script>

<MediaModalComponent
  bind:media_modal_idx
  bind:media_modal_media
  bind:media_modal_open
/>

<NewPostComponent {onPost} />

{#each feed as post}
  <PostComponent
    {ipfs_id}
    cid={null}
    {post}
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
