<script lang="ts">
  import {
    Button,
    Column,
    Link,
    OverflowMenu,
    OverflowMenuItem,
    ProgressBar,
    Row,
    Tile,
  } from "carbon-components-svelte";
  import Download from "carbon-icons-svelte/lib/Download.svelte";
  import PlayFilled from "carbon-icons-svelte/lib/PlayFilled.svelte";
  import TimeagoComponent from "./Timeago.svelte";
  import all from "it-all";
  import ext2mime from "ext2mime";
  import linkifyHtml from "linkify-html";
  import type { Media, Post } from "../types";
  import { concat } from "uint8arrays/concat";
  import { deletePost, ipfs, unfollowPublisher } from "../core";
  import { homeDir, join } from "@tauri-apps/api/path";
  import { onMount, onDestroy } from "svelte";
  import { save } from "@tauri-apps/api/dialog";
  import { stripHtml } from "string-strip-html";
  import { writeBinaryFile } from "@tauri-apps/api/fs";

  // import getVideoId from "get-video-id";
  // import { Player, Youtube, Dailymotion, Vimeo } from "@vime/svelte";

  export let media_modal_idx: number;
  export let media_modal_media: Media[];
  export let media_modal_open: boolean;

  export let removePostFromFeed: Function;
  export let ipfs_id: string;
  export let post: Post;

  let deleting: boolean = false;
  let media = [];

  let stripOpts = {
    onlyStripTags: ["script", "style", "xml", "sandbox"],
    stripTogetherWithTheirContents: ["script", "style", "xml", "sandbox"],
  };
  let bodyHTML = post.body;
  bodyHTML = stripHtml(bodyHTML, stripOpts).result.replace(/\n/g, "<br>");
  bodyHTML = linkifyHtml(bodyHTML, { target: "_blank" });

  // let links = extractLinks(post.body);
  // let video_ids = links
  //   .map((link) => getVideoId(link))
  //   .filter((link) => link.id);

  // function extractLinks(str, lower = false) {
  //   const regexp =
  //     /https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?!&//=]*)/gi;
  //   if (typeof str !== "string") {
  //     return [];
  //   }
  //   if (str) {
  //     let urls = str.match(regexp);
  //     if (urls) {
  //       return lower ? urls.map((item) => item.toLowerCase()) : urls;
  //     } else {
  //       return [];
  //     }
  //   } else {
  //     return [];
  //   }
  // }

  // function linkify(text) {
  //   const urlPattern =
  //     /(?:https?:)?\/\/(?:(?:[\w-]+\.)+[\w/#@~.-]*)(?:\?(?:[\w&=.!,;$#%-]+)?)?/gi;
  //   return (text || "").replace(urlPattern, function (url) {
  //     return `<a target="_blank" href="${url}">${url}</a>`;
  //   });
  // }

  function openMediaModal(idx) {
    console.log("openMediaModal");
    console.log(idx);
    media_modal_idx = idx;
    media_modal_media = media;
    media_modal_open = true;
  }

  async function removePostFromDbAndFeed(cid) {
    deleting = true;
    await deletePost(cid);
    removePostFromFeed(cid);
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
      thumbnail_for: null,
      filename: filename,
      content_type: fileType.mime,
    };
    if (isThumbnail) {
      mediaObj.thumbnail_for = filename;
      mediaObj.content_type = "image";
    }
    return mediaObj;
  }

  async function getMediaBinary(filename) {
    console.log("getMediaBinary");
    let path: string = post.cid + "/" + filename;
    return concat(await all(ipfs.cat(path)));
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

  onMount(async () => {
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
    // // this is required to avoid a memory leak,
    // // from posts which contain blob media...
    // media.forEach((mediaObj) => {
    //   if (mediaObj.element && mediaObj.element.src) {
    //     mediaObj.element.src = "";
    //     mediaObj.element.removeAttribute("src");
    //   }
    // });
    // media = media;
    media = [];
  });
</script>

{#if post}
  <Tile style="outline: 2px solid black">
    <div>
      {#if deleting}
        <ProgressBar helperText="Deleting..." />
      {:else}
        <OverflowMenu flipped style="float:right;">
          {#if post.publisher === ipfs_id}
            <OverflowMenuItem
              text="Delete post"
              on:click={() => {
                removePostFromDbAndFeed(post.cid);
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
      {/if}

      <Link size="lg" href="#/identity/{post.publisher}">
        {post.display_name || post.publisher}
      </Link>
      -
      <TimeagoComponent timestamp={post.timestamp} />
    </div>
    <br />
    {#if post.body || post.files}
      <div>
        {#if post.body}
          <div>
            {@html bodyHTML}
          </div>
          <!-- <div>
            {#each video_ids as link, idx (link)}
              <Player controls>
                {#if link.service === "dailymotion"}
                  <Dailymotion videoId={link.id} />
                {:else if link.service === "vimeo"}
                  <Vimeo videoId={link.id} />
                {:else if link.service === "youtube"}
                  <Youtube videoId={link.id} />
                {/if}
              </Player>
            {/each}
          </div> -->
          <br />
        {/if}
        <Row>
          {#each media as mediaObj, idx (mediaObj.filename)}
            <Column sm={8} md={8} lg={4}>
              {#if mediaObj.content_type}
                {#if mediaObj.content_type.includes("image")}
                  {#if mediaObj.thumbnail_for}
                    <div
                      bind:this={mediaObj.element}
                      on:click={() => loadVideo(mediaObj.filename, idx)}
                      on:keypress
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
                      on:keypress
                      src={mediaObj.url}
                    />
                  {/if}
                {:else if mediaObj.content_type.includes("audio")}
                  <audio
                    bind:this={mediaObj.element}
                    controls
                    src={mediaObj.url}
                  />
                {:else if mediaObj.content_type.includes("video")}
                  <video
                    bind:this={mediaObj.element}
                    controls
                    src={mediaObj.url}
                  >
                    <track kind="captions" />
                  </video>
                {:else if mediaObj.content_type.includes("pdf")}
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
