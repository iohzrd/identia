<script lang="ts">
  import MediaModalComponent from "./MediaModal.svelte";
  import NewPostComponent from "./NewPost.svelte";
  import PostComponent from "./Post.svelte";
  import { getPostFromDB, ipfs, select, updateFeed } from "../core";
  import type { IDResult } from "ipfs-core-types/src/root";
  import type { Post } from "../types";
  import { inview } from "svelte-inview/dist/";
  import { onMount, onDestroy } from "svelte";

  let ipfs_info: IDResult;
  let ipfs_id: string;
  let update_feed_interval = null;
  let limit: number = 10;
  let feed: Post[] = [];
  $: newest_ts = feed.length > 0 ? feed[0].timestamp : ts();
  $: oldest_ts = feed.length > 0 ? feed[feed.length - 1].timestamp : ts();
  $: feed_query = `SELECT posts.cid, posts.body, posts.files, posts.meta, posts.publisher, posts.timestamp, identities.display_name FROM posts INNER JOIN identities ON identities.publisher = posts.publisher WHERE posts.timestamp < ${oldest_ts} ORDER BY posts.timestamp DESC LIMIT ${limit}`;
  $: new_posts_query = `SELECT posts.cid, posts.body, posts.files, posts.meta, posts.publisher, posts.timestamp, identities.display_name FROM posts INNER JOIN identities ON identities.publisher = posts.publisher WHERE posts.publisher != '${ipfs_id}' AND posts.timestamp > ${newest_ts} ORDER BY posts.timestamp DESC`;

  let media_modal_idx = 0;
  let media_modal_media = [];
  let media_modal_open = false;

  function ts() {
    return new Date().getTime();
  }

  async function getFeedPage() {
    console.log("getFeedPage: ", feed_query);
    let page: Post[] = await select(feed_query);
    if (page.length > 0) {
      feed = [...feed, ...page];
    }
  }

  function onPost(post: Post) {
    console.log("onPost: ", post);
    feed = [post, ...feed];
  }

  async function getFeed() {
    console.log("getFeed: ", new_posts_query);
    await updateFeed();
    let new_posts: Post[] = await select(new_posts_query);
    if (new_posts.length > 0) {
      // the filter ensures posts from onPost and getFeed don't collide, which would cause an error.
      feed = [
        ...new_posts.filter((post) => post.publisher != ipfs_id),
        ...feed,
      ];
    }
  }

  onMount(async () => {
    ipfs_info = await ipfs.id();
    ipfs_id = ipfs_info.id;
    getFeedPage();
    update_feed_interval = setInterval(getFeed, 60 * 1000);
  });

  onDestroy(() => {
    clearInterval(update_feed_interval);
  });
</script>

<NewPostComponent {onPost} />

<!-- keyed each block required for reactivity... -->
{#each feed as post (post.cid)}
  <PostComponent
    {ipfs_id}
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

<MediaModalComponent
  bind:media_modal_idx
  bind:media_modal_media
  bind:media_modal_open
/>
