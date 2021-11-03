<script lang="ts">
  import type { PostResponse, MediaObj } from "../types.type";
  import { Buffer } from "buffer/index";
  import {
    Column,
    Grid,
    Link,
    Loading,
    OverflowMenu,
    OverflowMenuItem,
    Row,
    Tile,
  } from "carbon-components-svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/shell";
  import { onMount, onDestroy } from "svelte";
  import { create } from "ipfs-http-client/index";
  import linkifyStr from "linkify-string";
  import linkifyHtml from "linkify-html";
  import { stripHtml } from "string-strip-html";

  export let cid: String;
  export let postResponse: PostResponse;
  let media = [];

  let linkOptions = {
    target: "_blank",
  };
  let bodyHTML = linkifyStr(
    stripHtml(postResponse.post.body).result,
    linkOptions
  ).replace(/\n/g, "<br>");

  async function openWithDefaultApp(url) {
    await open(url);
  }

  async function getPostFromCid() {
    console.log("getPostFromCid");
    postResponse = await invoke("get_post_ipfs", {
      cid: cid,
    });
  }

  async function getMediaObject(filename) {
    console.log("getMediaObject");
    let bufs = [];
    const root_cid = postResponse.cid || cid;
    const path: string = root_cid + "/" + filename;
    const ipfs = await create("/ip4/127.0.0.1/tcp/5001");
    for await (const buf of ipfs.cat(path)) {
      bufs.push(buf);
    }
    const buf: Buffer = Buffer.concat(bufs);
    const mime: string = await invoke("get_mime", {
      buf: buf.slice(0, 16),
    });
    const blob = new Blob([buf], { type: mime });
    const urlCreator = window.URL || window.webkitURL;
    const mediaObj: MediaObj = {
      blobUrl: urlCreator.createObjectURL(blob),
      mime: mime,
      element: null,
    };
    return mediaObj;
  }

  onMount(async () => {
    if (!postResponse) {
      await getPostFromCid();
    }

    for await (const filename of postResponse.post.files) {
      media = [...media, await getMediaObject(filename)];
    }
  });

  onDestroy(() => {
    // this is required to avoid a memory leak...
    media.forEach((mediaObj) => {
      if (mediaObj.element && mediaObj.element.src) {
        mediaObj.element.src = "";
        mediaObj.element.removeAttribute("src");
      }
    });
    media = media;
    media = [];
  });
</script>

{#if postResponse.post}
  <Tile style="outline: 2px solid black">
    <div>
      <Link href="#/identity/{postResponse.post.publisher}">
        {postResponse.display_name || postResponse.post.publisher}
      </Link>
      -
      {new Date(Number(postResponse.post.timestamp)).toLocaleString()}

      <OverflowMenu flipped style="float:right;">
        <OverflowMenuItem text="Delete post" />
      </OverflowMenu>
    </div>
    <br />
    {#if postResponse.post.body || postResponse.post.files}
      <Grid fullWidth>
        {#if postResponse.post.body}
          <div>
            <!-- {@html linkifyStr(postResponse.post.body.replace(/\n/g, "<br>"))} -->
            {@html bodyHTML}
          </div>
          <br />
        {/if}
        {#if postResponse.post.files.length > 0 && media.length == 0}
          <Loading withOverlay={false} />
        {:else if postResponse.post.files.length > 0 && media.length > 0}
          <Row>
            {#each media as mediaObj}
              <Column sm={4} md={4} lg={4}>
                {#if mediaObj.mime && mediaObj.mime.includes("image")}
                  <img
                    class="thumbnail"
                    src={mediaObj.blobUrl}
                    alt=""
                    bind:this={mediaObj.element}
                  />
                {:else if mediaObj.mime && mediaObj.mime.includes("audio")}
                  <video
                    class="thumbnail"
                    src={mediaObj.blobUrl}
                    height="300"
                    controls
                    bind:this={mediaObj.element}
                  >
                    <track kind="captions" />
                  </video>
                {:else if mediaObj.mime && mediaObj.mime.includes("video")}
                  <video
                    src={mediaObj.blobUrl}
                    height="300"
                    controls
                    bind:this={mediaObj.element}
                  >
                    <track kind="captions" />
                  </video>
                {/if}
              </Column>
            {/each}
          </Row>
        {/if}
      </Grid>
    {/if}
  </Tile>
{/if}

<style>
  .thumbnail {
    width: auto;
    height: 200px;
    object-fit: cover;
  }
</style>
