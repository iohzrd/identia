<script lang="ts">
  import type { Post, PostResponse, MediaResponse } from "../types.type";
  import { Buffer } from "buffer/index";
  import { ClickableTile } from "carbon-components-svelte";
  import { Link } from "carbon-components-svelte";
  // import { Player, Video, DefaultUi } from "@vime/svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  // let player: Player;
  export let cid: String;
  export let postResponse: PostResponse;
  export let includeFrom: Boolean = true;
  $: post = postResponse.post;

  async function getPostFromCid() {
    console.log("getPostFromCid");
    await invoke("get_post_ipfs", {
      cid: cid,
    })
      .then((pr: PostResponse) => {
        console.log(pr);
        if (pr && pr.cid && pr.post) {
          postResponse = pr;
        }
      })
      .catch((err) => {
        console.log(err);
      });
  }

  async function getFile(filename) {
    console.log("getFile");
    const root_cid = postResponse.cid || cid;
    const path = root_cid + "/" + filename;
    console.log("path");
    console.log(path);
    const mr: MediaResponse = await invoke("get_file_ipfs", {
      cid: path,
    });
    const buf = Buffer.from(mr.data);
    const blob = new Blob([buf], { type: mr.mime });
    const urlCreator = window.URL || window.webkitURL;
    return {
      blob: urlCreator.createObjectURL(blob),
      mime: mr.mime,
    };
  }

  onMount(async () => {
    if (!postResponse) {
      await getPostFromCid();
    }
  });

  onDestroy(() => {});
</script>

<ClickableTile>
  {#if post}
    {#if post.body}
      <div>
        {@html post.body.replace(/\n/g, "<br>")}
        <!-- {post.body} -->
      </div>
    {/if}
    {#if post.files}
      {#each post.files as file}
        {#await getFile(file)}
          <p>file loading...</p>
        {:then media}
          <img src={media.blob} alt="" />
        {:catch error}
          <p style="color: red">{error.message}</p>
        {/await}
      {/each}
    {/if}
    {#if post.publisher && includeFrom}
      <div>
        publisher: <Link href="#/identity/{post.publisher}">
          {post.publisher}
        </Link>
      </div>
    {/if}
    {#if post.timestamp}
      <div>
        {post.timestamp}
      </div>
    {/if}
  {/if}
</ClickableTile>
