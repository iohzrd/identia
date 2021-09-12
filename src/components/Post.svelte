<script lang="ts">
  import { ClickableTile } from "carbon-components-svelte";
  import { Link } from "carbon-components-svelte";

  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  import type { Post, PostResponse } from "../types.type";

  export let cid: String;
  export let postResponse: PostResponse;
  export let includeFrom: Boolean = true;

  let post: Post;

  function onPostResponseObject(pr: PostResponse) {
    console.log("onPostObject");
    console.log(pr);
    if (pr && pr.cid && pr.post) {
      postResponse = pr;
    }
  }

  function getPostFromCid() {
    invoke("get_post_ipfs", {
      cid: cid,
    })
      .then(onPostResponseObject)
      .catch(onPostResponseObject);
  }

  onMount(async () => {
    console.log("Post");
    console.log(postResponse);
    if (!postResponse) {
      getPostFromCid();
    }
  });

  onDestroy(() => {});
</script>

<ClickableTile light>
  {#if postResponse}
    <div>
      {postResponse.cid}
    </div>
    <div>
      {#if postResponse.post && postResponse.post.body}
        {postResponse.post.body}
      {/if}
    </div>
    <div>
      {#if postResponse.post && postResponse.post.publisher && includeFrom}
        publisher: <Link href="#/identity/{postResponse.post.publisher}"
          >{postResponse.post.publisher}</Link
        >
      {/if}
    </div>
    <div>
      {#if postResponse.post && postResponse.post.ts}
        {postResponse.post.ts}
      {/if}
    </div>
  {/if}
</ClickableTile>
