<script lang="ts">
  import NewPostComponent from "$lib/NewPost.svelte";
  import PostComponent from "$lib/Post.svelte";
  import type { PageData } from "./$types";
  import type { Post } from "$lib/types";
  import { Tile } from "carbon-components-svelte";
  import { getPostFromDB } from "$lib/core";
  import { onMount, onDestroy } from "svelte";

  export let data: PageData;
  let post: Post | null = null;
  let show_comments: boolean = true;

  onMount(async () => {
    console.log("Post.onMount", show_comments);
    if (!post) {
      post = await getPostFromDB(data.cid);
    }
  });

  onDestroy(() => {});
</script>

{#if post}
  <PostComponent {post} {show_comments}>
    {data.cid}
  </PostComponent>
{/if}
