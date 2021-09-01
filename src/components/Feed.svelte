<script lang="ts">
  import {
    Button,
    ButtonSet,
    ButtonSkeleton,
    Column,
    Form,
    FormGroup,
    Grid,
    Row,
    TextInput,
    TextInputSkeleton,
    Tile,
  } from "carbon-components-svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";
  import NewPost from "./NewPost.svelte";
  import Post from "./Post.svelte";

  export let ipfs_id: string;

  let feed = [];
  let oldest_ts = 0;
  function onFeedPage(obj) {
    console.log("onFeedPage");
    console.log(obj);
  }

  function getFeedPage() {
    console.log(`getFeedPage: ${ipfs_id}`);

    if (feed.length > 0) {
      oldest_ts = feed[feed.length - 1].ts;
    } else {
      oldest_ts = Math.floor(new Date().getTime());
    }

    invoke("get_feed", {
      publisher: "ANY",
      tsop: "<=",
      // ts: oldest_ts,
      ts: 1630470260380,
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

  {#if feed}
    {#each feed as cid}
      <div>
        <Post {cid} includeFrom={true} />
      </div>
    {/each}
  {/if}
</Tile>
