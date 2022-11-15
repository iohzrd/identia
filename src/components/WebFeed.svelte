<script lang="ts">
  import MediaModalComponent from "./MediaModal.svelte";
  import WebFeedEntriesComponent from "./WebFeedEntries.svelte";
  import type { IDResult } from "ipfs-core-types/src/root";
  import type { WebFeedEntry, WebFeed, Media } from "../types";
  import { inview } from "svelte-inview/dist/";
  import { invoke } from "@tauri-apps/api";
  import { ipfs, select, updateFeed } from "../core";
  import { onMount, onDestroy } from "svelte";

  let ipfs_info: IDResult;
  let ipfs_id: string;
  let update_feed_interval = null;
  let limit: number = 10;
  let feed: WebFeedEntry[] = [];
  $: newest_ts = feed.length > 0 ? feed[0].timestamp : 0;
  $: oldest_ts = feed.length > 0 ? feed[feed.length - 1].timestamp : ts();
  $: web_feed_query = `SELECT posts.cid, posts.body, posts.files, posts.meta, posts.publisher, posts.timestamp, identities.display_name FROM posts INNER JOIN identities ON identities.publisher = posts.publisher WHERE posts.timestamp < ${oldest_ts} ORDER BY posts.timestamp DESC LIMIT ${limit}`;
  $: new_posts_query = `SELECT posts.cid, posts.body, posts.files, posts.meta, posts.publisher, posts.timestamp, identities.display_name FROM posts INNER JOIN identities ON identities.publisher = posts.publisher WHERE posts.publisher != '${ipfs_id}' AND posts.timestamp > ${newest_ts} ORDER BY posts.timestamp DESC`;

  let media_modal_idx: number = 0;
  let media_modal_media: Media[] = [];
  let media_modal_open: boolean = false;

  function ts() {
    return new Date().getTime();
  }

  async function getWebFeedPage() {
    // yt browse_id
    let test_urls = [
      "https://www.youtube.com/feeds/videos.xml?channel_id=UC4w1YQAJMWOz4qtxinq55LQ",
      "https://www.youtube.com/feeds/videos.xml?channel_id=UCfV0_wbjG8KJADuZT2ct4SA",
    ];

    test_urls.forEach(async (url) => {
      const ret: WebFeed = await invoke("fetch_external", {
        url: url,
      });
      console.log(ret);
      feed = [...feed, ...ret.entries];
      feed = feed.sort((a, b) => b.timestamp - a.timestamp);
    });
  }

  function insertPostIntoFeed(post: WebFeedEntry) {
    console.log("insertPostIntoFeed: ", post);
    feed = [post, ...feed];
  }

  function removePostFromFeed(cid: string) {
    console.log("removePostFromFeed: ", cid);
    feed = feed.filter((post) => post.cid != cid);
  }

  // async function getWebFeed() {
  //   console.log("getWebFeed: ", new_posts_query);
  //   await updateFeed();
  //   let new_posts: WebFeedEntry[] = await select(new_posts_query);
  //   if (new_posts.length > 0) {
  //     // the filter ensures posts from onPost and getWebFeed don't collide, which would cause an error.
  //     feed = [
  //       ...new_posts.filter((post) => post.publisher != ipfs_id),
  //       ...feed,
  //     ];
  //   }
  // }

  onMount(async () => {
    // ipfs_info = await ipfs.id();
    // ipfs_id = ipfs_info.id.toString();
    getWebFeedPage();
    // update_feed_interval = setInterval(getWebFeed, 60 * 1000);
  });

  onDestroy(() => {
    // clearInterval(update_feed_interval);
  });
</script>

<!-- keyed each block required for reactivity... -->
{#each feed as entry (entry.cid)}
  <WebFeedEntriesComponent
    {removePostFromFeed}
    {ipfs_id}
    {entry}
    bind:media_modal_idx
    bind:media_modal_media
    bind:media_modal_open
  />
{/each}

<!-- {#if feed.length >= limit}
  <div
    use:inview={{}}
    on:enter={(event) => {
      if (event.detail.inView) {
        getWebFeedPage();
      }
    }}
  />
{/if} -->

<MediaModalComponent
  bind:media_modal_idx
  bind:media_modal_media
  bind:media_modal_open
/>
