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
  import Download from "carbon-icons-svelte/lib/Download.svelte";
  import PlayFilled from "carbon-icons-svelte/lib/PlayFilled.svelte";
  import ext2mime from "ext2mime";
  import linkifyStr from "linkify-string";
  import type { Media, Post } from "../types";
  import { concat } from "uint8arrays/concat";
  import { deletePost, ipfs, unfollowPublisher } from "../core";
  import { format as formatTime } from "timeago.js";
  import { homeDir, join } from "@tauri-apps/api/path";
  import { onMount, onDestroy } from "svelte";
  import { save } from "@tauri-apps/api/dialog";
  import { stripHtml } from "string-strip-html";
  import { writeBinaryFile } from "@tauri-apps/api/fs";

  export let media_modal_idx: number;
  export let media_modal_media: Media[];
  export let media_modal_open: boolean;

  export let ipfs_id: string;
  export let post: Post;

  let timeout;
  let timeout_time = 1000;
  let timeago: string = formatTime(post.timestamp);
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

  async function getMedia(filename, isThumbnail = false) {
    console.log("getMedia");
    let cid = post.cid;
    if (!post.cid.includes("ipfs/")) {
      cid = "ipfs/" + post.cid;
    }
    const path: string = "http://127.0.0.1:8080/" + cid + "/" + filename;
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

  async function getMediaBinary(filename) {
    console.log("getMediaBinary");
    let path: string = post.cid + "/" + filename;
    const bufs = [];
    for await (const buf of ipfs.cat(path)) {
      bufs.push(buf);
    }
    return concat(bufs);
  }

  async function getMediaBlob(filename) {
    console.log("getMediaBlob");
    const fileType = {
      ext: filename.split(".").pop(),
      mime: ext2mime(filename.split(".").pop()),
    };
    const buf = await getMediaBinary(filename);
    return new Blob([buf], { type: fileType.mime });
  }

  async function getMediaBlobUrl(filename) {
    console.log("getMediaBlobUrl");
    const blob = await getMediaBlob(filename);
    const urlCreator = window.URL || window.webkitURL;
    return urlCreator.createObjectURL(blob);
  }

  async function saveMedia(filename) {
    console.log("saveMedia");
    const home = await homeDir();
    const path = await join(home, filename);
    const user_path = await save({
      defaultPath: path,
    });
    await writeBinaryFile({
      path: user_path,
      contents: await getMediaBinary(filename),
    });
  }

  async function loadVideo(filename, idx: number) {
    console.log("loadVideo: ", idx);
    media[idx] = await getMedia(filename);
    media = media;
  }

  function newTimeout() {
    timeago = formatTime(post.timestamp);
    let delta = new Date().getTime() - post.timestamp;
    if (delta < 60 * 1000) {
      // less than a minue, update once a second
      timeout_time = 1000;
    } else if (delta < 60 * 60 * 1000) {
      // less than an hour, update once a minute
      timeout_time = 60 * 1000;
    } else {
      // update once an hour
      timeout_time = 60 * 60 * 1000;
    }
    timeout = setTimeout(newTimeout, timeout_time);
  }

  onMount(async () => {
    timeout = setTimeout(newTimeout, timeout_time);

    for await (const filename of post.files) {
      const is_video = ext2mime(filename.split(".").pop()).includes("video");
      console.log("isVideo");
      console.log(is_video);
      if (is_video) {
        // get thumbnail here...
        media = [...media, await getMedia(filename, true)];
      } else {
        media = [...media, await getMedia(filename)];
      }
    }
  });

  onDestroy(() => {
    clearTimeout(timeout);
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
      - {timeago} ({datetime})

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
      <div>
        {#if post.body}
          <div>
            {@html bodyHTML}
          </div>
          <br />
        {/if}
        <Row>
          {#each media as mediaObj, idx (mediaObj.filename)}
            <Column sm={8} md={8} lg={4}>
              {#if mediaObj.mime}
                {#if mediaObj.mime.includes("image")}
                  {#if mediaObj.thumbnailFor}
                    <div
                      bind:this={mediaObj.element}
                      on:click={() => loadVideo(mediaObj.filename, idx)}
                    >
                      <PlayFilled size={32} />
                    </div>

                    <!-- <img
                      alt=""
                      bind:this={mediaObj.element}
                      on:click={() => loadVideo(mediaObj, idx)}
                      src={mediaObj.url}
                    /> -->
                  {:else}
                    <img
                      alt=""
                      bind:this={mediaObj.element}
                      on:click={() => openMediaModal(idx)}
                      src={mediaObj.url}
                    />
                  {/if}
                {:else if mediaObj.mime.includes("audio")}
                  <audio
                    bind:this={mediaObj.element}
                    controls
                    src={mediaObj.url}
                  />
                {:else if mediaObj.mime.includes("video")}
                  <video
                    bind:this={mediaObj.element}
                    controls
                    src={mediaObj.url}
                  >
                    <track kind="captions" />
                  </video>
                {:else if mediaObj.mime.includes("pdf")}
                  <Button
                    download={mediaObj.filename}
                    href={mediaObj.url}
                    icon={Download}
                    iconDescription="Download"
                    kind="secondary"
                    on:click={() => saveMedia(mediaObj.filename)}
                  >
                    {mediaObj.filename}
                  </Button>
                {/if}
              {/if}
            </Column>
          {/each}
        </Row>
      </div>
    {/if}
  </Tile>
{/if}

<style>
  img {
    height: 150px;
    width: 150px;
    object-fit: cover;
  }

  video {
    height: auto;
    width: 100%;
    object-fit: contain;
  }
</style>
