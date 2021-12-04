<script lang="ts">
  import {
    Button,
    Column,
    Grid,
    Link,
    OverflowMenu,
    OverflowMenuItem,
    Row,
    Tile,
  } from "carbon-components-svelte";
  import DocumentPdf32 from "carbon-icons-svelte/lib/DocumentPdf32";
  import * as timeago from "timeago.js";
  import linkifyStr from "linkify-string";
  import type { MediaObj, PostResponse } from "../types.type";
  import { Buffer } from "buffer/index";
  import { create } from "ipfs-http-client/index";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";
  import { stripHtml } from "string-strip-html";

  export let cid: String;
  export let post_response: PostResponse;
  export let media_modal_idx: number;
  export let media_modal_media: MediaObj[];
  export let media_modal_open: boolean;

  let media = [];
  let linkOptions = {
    target: "_blank",
  };
  let bodyHTML = linkifyStr(
    stripHtml(post_response.post.body).result,
    linkOptions
  ).replace(/\n/g, "<br>");

  async function getPostFromCid() {
    console.log("getPostFromCid");
    post_response = await invoke("get_post_ipfs", {
      cid: cid,
    });
  }

  async function getMediaObject(filename) {
    console.log("getMediaObject");
    let bufs = [];
    const root_cid = post_response.cid || cid;
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
      element: null,
      filename: filename,
      mime: mime,
    };
    return mediaObj;
  }

  function openMediaModal(idx) {
    console.log("openMediaModal");
    console.log(idx);
    media_modal_idx = idx;
    media_modal_media = media;
    media_modal_open = true;
  }

  onMount(async () => {
    if (!post_response) {
      await getPostFromCid();
    }

    for await (const filename of post_response.post.files) {
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

{#if post_response.post}
  <Tile style="outline: 2px solid black">
    <div>
      <Link href="#/identity/{post_response.post.publisher}">
        {post_response.display_name || post_response.post.publisher}
      </Link>
      - {timeago.format(post_response.post.timestamp)} ({new Date(
        Number(post_response.post.timestamp)
      ).toLocaleString()})

      <OverflowMenu flipped style="float:right;">
        <OverflowMenuItem text="Delete post" />
      </OverflowMenu>
    </div>
    <br />
    {#if post_response.post.body || post_response.post.files}
      <Grid fullWidth>
        {#if post_response.post.body}
          <div>
            <!-- {@html linkifyStr(post_response.post.body.replace(/\n/g, "<br>"))} -->
            {@html bodyHTML}
          </div>
          <br />
        {/if}
        <Row>
          {#each media as mediaObj, idx}
            <Column sm={4} md={4} lg={4}>
              {#if mediaObj.mime && mediaObj.mime.includes("image")}
                <img
                  class="thumbnail"
                  src={mediaObj.blobUrl}
                  alt=""
                  bind:this={mediaObj.element}
                  on:click={() => openMediaModal(idx)}
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
              {:else if mediaObj.mime && mediaObj.mime.includes("pdf")}
                <Button kind="secondary" on:click={() => openMediaModal(idx)}>
                  {mediaObj.filename}
                  <DocumentPdf32 />
                </Button>
              {/if}
            </Column>
          {/each}
        </Row>
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
