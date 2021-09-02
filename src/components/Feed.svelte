<script lang="ts">
  import { Tile } from "carbon-components-svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  import NewPost from "./NewPost.svelte";
  import Post from "./Post.svelte";
  import type { PostResponse } from "../types.type";
  export let ipfs_id: string;

  let feed: PostResponse[] = [];
  let oldest_ts = 0;
  function onFeedPage(obj) {
    console.log("onFeedPage");
    console.log(obj);
    obj.forEach((element) => {
      console.log("element");
      console.log(element);
      feed = [element, ...feed];
    });
  }

  function getFeedPage() {
    console.log(`getFeedPage: ${ipfs_id}`);

    if (feed.length > 0) {
      oldest_ts = feed[feed.length - 1].post.ts;
    } else {
      oldest_ts = Math.floor(new Date().getTime());
    }

    invoke("get_feed", {
      publisher: "ANY",
      tsop: "<=",
      ts: oldest_ts,
      limit: 10,
    })
      .then(onFeedPage)
      .catch(onFeedPage);
  }

  onMount(async () => {
    getFeedPage();
  });

  onDestroy(() => {});
</script>

<Tile>
  <NewPost />

  {#each feed as postResponse}
    <Post cid={null} {postResponse} />
  {/each}
</Tile>
