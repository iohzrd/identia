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
  import DocumentPdf from "carbon-icons-svelte/lib/DocumentPdf.svelte";
  import PlayFilled from "carbon-icons-svelte/lib/PlayFilled.svelte";
  import { deletePost, unfollowPublisher } from "../core";
  import ext2mime from "ext2mime";
  import linkifyStr from "linkify-string";
  import type { Media, Post } from "../types";
  import { format as formatTime } from "timeago.js";
  import { onMount, onDestroy } from "svelte";
  import { stripHtml } from "string-strip-html";

  export let media_modal_idx: number;
  export let media_modal_media: Media[];
  export let media_modal_open: boolean;

  export let ipfs_id: string;
  export let post: Post;

  let timer;
  let timestamp: string = formatTime(post.timestamp);
  let datetime: string = new Date(post.timestamp).toLocaleString();
  let media = [];
  let bodyHTML = linkifyStr(stripHtml(post.body).result, {
    target: "_blank",
  }).replace(/\n/g, "<br>");

  function openMediaModal(idx) {
    console.log("openMediaModal");
    console.log(idx);
    media_modal_idx = idx;
    media_modal_media = media;
    media_modal_open = true;
  }

  async function getMediaObject(filename, isThumbnail = false) {
    console.log("getMediaObject");
    let cid = post.cid;
    if (!post.cid.includes("ipfs/")) {
      cid = "ipfs/" + post.cid;
    }
    const path: string = "http://localhost:8080/" + cid + "/" + filename;
    const fileType = {
      ext: filename.split(".").pop(),
      mime: ext2mime(filename.split(".").pop()),
    };
    let mediaObj: Media = {
      url: path,
      element: null,
      thumbnailFor: null,
      filename: filename,
      mime: fileType.mime,
    };
    if (isThumbnail) {
      mediaObj.thumbnailFor = filename;
      mediaObj.mime = "image";
    }
    return mediaObj;
  }

  async function loadVideo(filename, idx: number) {
    console.log("loadVideo: ", idx);
    media[idx] = await getMediaObject(filename);
    media = media;
  }

  onMount(async () => {
    timer = setInterval(() => {
      timestamp = formatTime(post.timestamp);
    }, 60000);

    for await (const filename of post.files) {
      const is_video = ext2mime(filename.split(".").pop()).includes("video");
      console.log("isVideo");
      console.log(is_video);
      if (is_video) {
        // get thumbnail here...
        media = [...media, await getMediaObject(filename, true)];
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

{#if post}
  <Tile style="outline: 2px solid black">
    <div>
      <Link href="#/identity/{post.publisher}">
        {post.display_name || post.publisher}
      </Link>
      - {timestamp} ({datetime})

      <OverflowMenu flipped style="float:right;">
        {#if post.publisher === ipfs_id}
          <OverflowMenuItem
            text="Delete post"
            on:click={() => {
              deletePost(post.cid);
            }}
          />
        {:else}
          <OverflowMenuItem
            text="Unfollow publisher"
            on:click={() => {
              unfollowPublisher(post.publisher);
            }}
          />
        {/if}
      </OverflowMenu>
    </div>
    <br />
    {#if post.body || post.files}
      <Grid fullWidth>
        {#if post.body}
          <div>
            {@html bodyHTML}
          </div>
          <br />
        {/if}
        <Row>
          {#each media as mediaObj, idx}
            <Column sm={4} md={4} lg={4}>
              {#if mediaObj.mime}
                {#if mediaObj.mime.includes("image")}
                  {#if mediaObj.thumbnailFor}
                    <div
                      class="thumbnail"
                      bind:this={mediaObj.element}
                      on:click={() => loadVideo(mediaObj.filename, idx)}
                    >
                      <PlayFilled size={32} />
                    </div>

                    <!-- <img
                      class="thumbnail"
                      src={mediaObj.url}
                      alt=""
                      bind:this={mediaObj.element}
                      on:click={() => loadVideo(mediaObj, idx)}
                    /> -->
                  {:else}
                    <img
                      class="thumbnail"
                      src={mediaObj.url}
                      alt=""
                      bind:this={mediaObj.element}
                      on:click={() => openMediaModal(idx)}
                    />
                  {/if}
                {:else if mediaObj.mime.includes("audio")}
                  <audio
                    class="thumbnail"
                    src={mediaObj.url}
                    height="300"
                    controls
                    bind:this={mediaObj.element}
                  />
                {:else if mediaObj.mime.includes("video")}
                  <video
                    src={mediaObj.url}
                    height="300"
                    controls
                    bind:this={mediaObj.element}
                  >
                    <track kind="captions" />
                  </video>
                {:else if mediaObj.mime.includes("pdf")}
                  <Button kind="secondary" on:click={() => openMediaModal(idx)}>
                    {mediaObj.filename}
                    <DocumentPdf size={32} />
                  </Button>
                {/if}
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
