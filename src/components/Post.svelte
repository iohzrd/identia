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
  import ext2mime from "ext2mime";
  import linkifyStr from "linkify-string";
  import type { MediaObj, PostResponse } from "../types.type";
  import { Buffer } from "buffer/index";
  import { create } from "ipfs-http-client";
  import { format as formatTime } from "timeago.js";
  import { getPostFromIPFS, deletePost } from "../Core.svelte";
  import { onMount, onDestroy } from "svelte";
  import { stripHtml } from "string-strip-html";

  export let ipfs_id: string;
  export let cid: string;
  export let post_response: PostResponse;
  export let media_modal_idx: number;
  export let media_modal_media: MediaObj[];
  export let media_modal_open: boolean;
  const root_cid = post_response.cid || cid;

  let timer;
  let timestamp: string = formatTime(post_response.timestamp);
  let datetime: string = new Date(post_response.timestamp).toLocaleString();
  let media = [];
  let bodyHTML = linkifyStr(stripHtml(post_response.body).result, {
    target: "_blank",
  }).replace(/\n/g, "<br>");

  async function getMediaObject(filename, isThumbnail = false) {
    console.log("getMediaObject");
    let bufs = [];
    const path: string = root_cid + "/" + filename;
    const ipfs = await create({ url: "/ip4/127.0.0.1/tcp/5001" });
    for await (const buf of ipfs.cat(path)) {
      bufs.push(buf);
    }
    const buf: Buffer = Buffer.concat(bufs);
    const fileType = {
      ext: filename.split(".").pop(),
      mime: ext2mime(filename.split(".").pop()),
    };

    const blob = new Blob([buf], { type: fileType.mime });
    const urlCreator = window.URL || window.webkitURL;
    const mediaObj: MediaObj = {
      blobUrl: urlCreator.createObjectURL(blob),
      element: null,
      thumbnailFor: "",
      filename: filename,
      mime: fileType.mime,
    };
    return mediaObj;
  }

  function isVideo(filename: string) {
    return ext2mime(filename.split(".").pop()).includes("video");
  }

  function openMediaModal(idx) {
    console.log("openMediaModal");
    console.log(idx);
    media_modal_idx = idx;
    media_modal_media = media;
    media_modal_open = true;
  }

  onMount(async () => {
    timer = setInterval(() => {
      timestamp = formatTime(post_response.timestamp);
    }, 60000);
    if (!post_response) {
      post_response = await getPostFromIPFS(cid);
    }

    for await (const filename of post_response.files) {
      console.log("isVideo");
      console.log(isVideo(filename));
      if (isVideo(filename)) {
        // get thumbnail here...
        const mediaObj: MediaObj = await getMediaObject(filename, true);
        media = [...media, mediaObj];
      } else {
        media = [...media, await getMediaObject(filename)];
      }
    }
  });

  onDestroy(() => {
    clearInterval(timer);
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

{#if post_response}
  <Tile style="outline: 2px solid black">
    <div>
      <Link href="#/identity/{post_response.publisher}">
        {post_response.display_name || post_response.publisher}
      </Link>
      - {timestamp} ({datetime})

      <OverflowMenu flipped style="float:right;">
        {#if post_response.publisher === ipfs_id}
          <OverflowMenuItem
            text="Delete post"
            on:click={() => {
              deletePost(root_cid);
            }}
          />
        {/if}
      </OverflowMenu>
    </div>
    <br />
    {#if post_response.body || post_response.files}
      <Grid fullWidth>
        {#if post_response.body}
          <div>
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
