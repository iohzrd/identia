<script lang="ts">
  import {
    Button,
    Column,
    Link,
    OverflowMenu,
    OverflowMenuItem,
    ProgressBar,
    Row,
    TextArea,
    Tile,
  } from "carbon-components-svelte";
  import CommentComponent from "$lib/Comment.svelte";
  import Download from "carbon-icons-svelte/lib/Download.svelte";
  import MediaModalComponent from "$lib/MediaModal.svelte";
  import PlayFilled from "carbon-icons-svelte/lib/PlayFilled.svelte";
  import Reply from "carbon-icons-svelte/lib/Reply.svelte";
  import ThumbsDown from "carbon-icons-svelte/lib/ThumbsDown.svelte";
  import ThumbsDownFilledfrom from "carbon-icons-svelte/lib/ThumbsDownFilled.svelte";
  import ThumbsUp from "carbon-icons-svelte/lib/ThumbsUp.svelte";
  import ThumbsUpFilled from "carbon-icons-svelte/lib/ThumbsUpFilled.svelte";
  import TimeagoComponent from "$lib/Timeago.svelte";
  import all from "it-all";
  import ext2mime from "ext2mime";
  import linkifyHtml from "linkify-html";
  import type { Media, Post } from "$lib/types";
  import type { MessageExtended } from "$lib/types";
  import { concat } from "uint8arrays/concat";
  import { deletePost, ipfs, unfollowPublisher } from "$lib/core";
  import { homeDir, join } from "@tauri-apps/api/path";
  import { onMount, onDestroy } from "svelte";
  import { pubsubStore, globalPubsubHandler } from "$lib/pubsub";
  import { save } from "@tauri-apps/api/dialog";
  import { select } from "./db";
  import { stripHtml } from "string-strip-html";
  import { writeBinaryFile } from "@tauri-apps/api/fs";

  // import getVideoId from "get-video-id";
  // import { Player, Youtube, Dailymotion, Vimeo } from "@vime/svelte";

  export let removePostFromFeed: Function = () => {};
  export let ipfs_id: string = "";
  export let post: Post;
  export let show_comments: boolean = false;

  let unsubscribe: any;

  // media modal props...
  let media_modal_idx = 0;
  let media_modal_media: Media[] = [];
  let media_modal_open = false;

  let deleting: boolean = false;
  let media: Media[] = [];

  let comments: MessageExtended[] = [];
  let replying = false;
  let reply: string = "";

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

  function openMediaModal(idx: number) {
    media_modal_open = true;
    media_modal_idx = idx;
    media_modal_media = media.filter((m) => m.filename != "post.json");
  }

  async function removePostFromDbAndFeed(cid: string) {
    deleting = true;
    await deletePost(cid);
    removePostFromFeed(cid);
  }

  async function getMedia(filename: string, isThumbnail = false) {
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

  async function getMediaBinary(filename: string) {
    let path: string = post.cid + "/" + filename;
    return concat(await all(ipfs.cat(path)));
  }

  async function getMediaBlob(filename: string) {
    const fileType = {
      ext: filename.split(".").pop(),
      mime: ext2mime(filename.split(".").pop()),
    };
    const buf = await getMediaBinary(filename);
    return new Blob([buf], { type: fileType.mime });
  }

  async function getMediaBlobUrl(filename: string) {
    const blob = await getMediaBlob(filename);
    const urlCreator = window.URL || window.webkitURL;
    return urlCreator.createObjectURL(blob);
  }

  async function saveMedia(filename: string) {
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

  async function loadVideo(filename: string, idx: number) {
    media[idx] = await getMedia(filename);
    media = media;
  }

  async function postReply() {
    await ipfs.pubsub.publish(
      post.publisher,
      new TextEncoder().encode(
        JSON.stringify({
          body: reply,
          inReplyTo: post.cid,
        })
      )
    );
    reply = "";
    replying = false;
  }

  function cancelReply() {
    reply = "";
    replying = false;
  }

  async function messageHandler(message: MessageExtended) {
    comments = [message, ...comments];

    // await execute(
    //   "INSERT INTO comments (data,from,inReplyTo,key,sequenceNumber,signature,timestamp,topic,type) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)",
    //   [
    //     message.data,
    //     message.from,
    //     message.inReplyTo,
    //     message.key,
    //     String(message.sequenceNumber),
    //     message.signature,
    //     message.timestamp,
    //     message.topic,
    //     message.type,
    //   ]
    // );
  }

  onMount(async () => {
    if (show_comments) {
      const activeSubs = await ipfs.pubsub.ls();
      if (!activeSubs.includes(post.publisher)) {
        await ipfs.pubsub.subscribe(post.publisher, globalPubsubHandler);
      }

      unsubscribe = pubsubStore.subscribe(
        post.publisher,
        post.cid,
        messageHandler
      );
    }

    // comments = await select(
    //   "SELECT (SequenceNumber) FROM comments WHERE inReplyTo = ?",
    //   [post.cid]
    // );

    // if (typeof post.files === "string") {
    //   post.files = JSON.parse(post.files);
    // }

    for await (const filename of post.files) {
      const is_video = ext2mime(filename.split(".").pop()).includes("video");
      if (is_video) {
        // get thumbnail here...
        media = [...media, await getMedia(filename, true)];
      } else {
        media = [...media, await getMedia(filename)];
      }
    }

    // if (show_comments) {
    //   for (let index = 0; index < 100; index++) {
    //     await ipfs.pubsub.publish(
    //       post.publisher,
    //       createComment(post.cid, String(index))
    //       // new TextEncoder().encode(
    //       //   JSON.stringify({
    //       //     inReplyTo: post.cid,
    //       //     body: String(index),
    //       //   })
    //       // )
    //     );
    //   }
    // }
  });

  onDestroy(async () => {
    if (post.publisher != ipfs_id) {
      const activeSubs = await ipfs.pubsub.ls();
      if (activeSubs.includes(post.publisher)) {
        await ipfs.pubsub.unsubscribe(post.publisher, globalPubsubHandler);
      }
    }

    if (unsubscribe != undefined) {
      unsubscribe();
    }
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

{#if media_modal_open}
  <MediaModalComponent
    bind:start={media_modal_idx}
    bind:media={media_modal_media}
    bind:open={media_modal_open}
  />
{/if}

{#if post}
  <Tile style="outline: 2px solid black">
    <div>
      {#if deleting}
        <ProgressBar helperText="Deleting..." />
      {:else}
        <OverflowMenu flipped style="float:right;">
          <OverflowMenuItem text="Comments" href="/post/{post.cid}" />
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

      <Link size="lg" href="/identity/{post.publisher}">
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

    <br />

    {#if true}
      <Button
        disabled
        icon={ThumbsUp}
        iconDescription="Like"
        kind="ghost"
        size="small"
      />
    {:else}
      <Button
        disabled
        icon={ThumbsUpFilled}
        iconDescription="Like"
        kind="ghost"
        size="small"
      />
    {/if}

    {#if true}
      <Button
        disabled
        icon={ThumbsDown}
        iconDescription="Dislike"
        kind="ghost"
        size="small"
      />
    {:else}
      <Button
        disabled
        icon={ThumbsDownFilledfrom}
        iconDescription="Dislike"
        kind="ghost"
        size="small"
      />
    {/if}

    <Button
      icon={Reply}
      iconDescription="Reply"
      kind="ghost"
      size="small"
      on:click={() => {
        replying = !replying;
      }}
    />

    {#if replying}
      <div>
        <TextArea bind:value={reply} placeholder="Add a reply..." rows={2} />
        <Button
          kind="ghost"
          size="small"
          style="float:right;"
          on:click={postReply}>Reply</Button
        >
        <Button
          kind="ghost"
          size="small"
          style="float:right;"
          on:click={cancelReply}>Cancel</Button
        >
      </div>
      <br />
      <br />
    {/if}

    <br />

    {#if show_comments}
      {#each comments as comment (comment.sequenceNumber)}
        <CommentComponent {comment} />
      {/each}
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
