<script lang="ts">
  import { ClickableTile } from "carbon-components-svelte";
  import { Link } from "carbon-components-svelte";

  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  import type { Post, PostResponse } from "../types.type";

  export let cid: String;
  export let postResponse: PostResponse;
  export let includeFrom: Boolean = true;
  $: post = postResponse.post;

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

<ClickableTile>
  {#if post}
    {#if post && post.body}
      <div>
        <!-- {@html post.body.replace(/\n/g, "<br>")} -->
        {post.body}
      </div>
    {/if}
    {#if post && post.publisher && includeFrom}
      <div>
        publisher: <Link href="#/identity/{post.publisher}">
          {post.publisher}
        </Link>
      </div>
    {/if}
    {#if post && post.timestamp}
      <div>
        {post.timestamp}
      </div>
    {/if}
  {/if}
</ClickableTile>
