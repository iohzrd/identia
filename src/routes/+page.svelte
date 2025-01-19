<script lang="ts">
  import NewPostComponent from "$lib/NewPost.svelte";
  import PostComponent from "$lib/Post.svelte";
  import type { IDResult } from "kubo-rpc-client";
  import type { Post } from "$lib/types";
  import { inview } from "svelte-inview/dist/";
  import { ipfs, log, updateFeed } from "$lib/core";
  import { onMount, onDestroy } from "svelte";
  import { select } from "$lib/db";

  let ipfs_info: IDResult;
  let ipfs_id: string = $state("");
  let update_feed_interval: any = null;
  let limit: number = 10;
  let feed: Post[] = $state([]);

  let media_modal_idx = 0;
  let media_modal_media = [];
  let media_modal_open = false;

  let show_comments: boolean = false;

  function ts() {
    return new Date().getTime();
  }

  async function getFeedPage() {
    let page: Post[] = await select(feed_query);
    if (page.length > 0) {
      feed = [...feed, ...page];
    }
  }

  function insertPostIntoFeed(post: Post) {
    feed = [post, ...feed];
  }

  function removePostFromFeed(cid: string) {
    feed = feed.filter((post) => post.cid != cid);
  }

  async function getFeed() {
    await updateFeed();
    let new_posts: Post[] = await select(new_posts_query);
    console.log("new_posts: ", new_posts);
    if (new_posts.length > 0) {
      // the filter ensures posts from onPost and getFeed don't collide, which would cause an error.
      feed = [
        ...new_posts.filter((post) => post.publisher != ipfs_id),
        ...feed,
      ];
      console.log("feed: ", feed);
    }
  }

  onMount(async () => {
    ipfs_info = await ipfs.id();
    ipfs_id = ipfs_info.id.toString();
    getFeedPage();
    update_feed_interval = setInterval(getFeed, 60 * 1000);
  });

  onDestroy(() => {
    clearInterval(update_feed_interval);
  });
  let newest_ts = $derived(feed.length > 0 ? feed[0].timestamp : 0);
  let oldest_ts = $derived(
    feed.length > 0 ? feed[feed.length - 1].timestamp : ts()
  );
  let feed_query = $derived(
    `SELECT posts.cid, posts.body, posts.files, posts.meta, posts.publisher, posts.timestamp, identities.display_name FROM posts INNER JOIN identities ON identities.publisher = posts.publisher WHERE posts.timestamp < ${oldest_ts} ORDER BY posts.timestamp DESC LIMIT ${limit}`
  );
  let new_posts_query = $derived(
    `SELECT posts.cid, posts.body, posts.files, posts.meta, posts.publisher, posts.timestamp, identities.display_name FROM posts INNER JOIN identities ON identities.publisher = posts.publisher WHERE posts.publisher != '${ipfs_id}' AND posts.timestamp > ${newest_ts} ORDER BY posts.timestamp DESC`
  );
</script>

<NewPostComponent {insertPostIntoFeed} />

<!-- keyed each block required for reactivity... -->
{#each feed as post (post.cid)}
  <PostComponent {removePostFromFeed} {ipfs_id} {post} {show_comments} />
{/each}

{#if feed.length >= limit}
  <div
    use:inview={{}}
    onenter={(event) => {
      if (event.detail.inView) {
        getFeedPage();
      }
    }}
  ></div>
{/if}
