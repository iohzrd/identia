<script lang="ts">
  import {
    TextInputSkeleton,
    ClickableTile,
    Tile,
  } from "carbon-components-svelte";

  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  //   export let post;
  export let cid: String;
  export let includeFrom: Boolean = false;
  let post: Post;

  type Post = {
    aux: object[];
    body: string;
    files: string[];
    files_root: string;
    meta: string[];
    publisher: string;
    ts: number;
  };

  type PostResponse = {
    post: Post;
    cid: string;
  };

  function onPostObject(postResponse: PostResponse) {
    console.log("onPostObject");
    console.log(postResponse);
    if (postResponse && postResponse.post) {
      post = postResponse.post;
    }
  }

  function requestTestPost() {
    invoke("ipfs_get_post", {
      cid: cid,
    })
      .then(onPostObject)
      .catch(onPostObject);
  }

  onMount(async () => {
    requestTestPost();
  });

  onDestroy(() => {});
</script>

<ClickableTile light>
  <div>
    {cid}
  </div>
  <div>
    {#if post && post.body}
      {post.body}
    {/if}
  </div>
  <div>
    {#if post && post.publisher && includeFrom}
      From: {post.publisher}
    {/if}
  </div>
  <div>
    {#if post && post.ts}
      {post.ts}
    {/if}
  </div>
</ClickableTile>
