<script lang="ts">
  import CommentComponent from "$lib/Comment.svelte";
  import MediaModalComponent from "$lib/MediaModal.svelte";
  import TimeagoComponent from "$lib/Timeago.svelte";
  import type { Media, Post } from "$lib/types";
  import type { MessageExtended } from "$lib/types";
  import { createJson, createTopical } from "$lib/flatbuffers";
  import { deletePost, ipfs, unfollowPublisher } from "$lib/core";
  import { pubsubStore, globalPubsubHandler } from "$lib/pubsub";

  import { Button } from "flowbite-svelte";
  import { ButtonGroup } from "flowbite-svelte";
  import { Card } from "flowbite-svelte";
  import { Dropdown } from "flowbite-svelte";
  import { DropdownItem } from "flowbite-svelte";
  import { Progressbar } from "flowbite-svelte";
  import { Spinner } from "flowbite-svelte";
  import { Textarea } from "flowbite-svelte";

  import { DotsHorizontalOutline } from "flowbite-svelte-icons";
  import { ThumbsDownOutline } from "flowbite-svelte-icons";
  import { ThumbsDownSolid } from "flowbite-svelte-icons";
  import { ThumbsUpOutline } from "flowbite-svelte-icons";
  import { ThumbsUpSolid } from "flowbite-svelte-icons";
  import { DownloadSolid } from "flowbite-svelte-icons";
  import { PlaySolid } from "flowbite-svelte-icons";
  import { ReplySolid } from "flowbite-svelte-icons";
  import { ReplyOutline } from "flowbite-svelte-icons";

  import all from "it-all";
  import ext2mime from "ext2mime";
  import linkifyHtml from "linkify-html";
  import { concat } from "uint8arrays/concat";
  import { homeDir, join } from "@tauri-apps/api/path";
  import { onMount, onDestroy } from "svelte";
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
    console.log("openMediaModal", idx);
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
      // createTopical(post.cid, reply, [])
      createJson({
        body: reply,
        inReplyTo: post.cid,
      })
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

<Card class="w-full max-w-md">
  <div class="flex justify-end">
    {#if deleting}
      <Spinner />
      <!-- <Progressbar /> -->
    {:else}
      <DotsHorizontalOutline />
      <Dropdown>
        <DropdownItem href="/post/{post.cid}">Comments</DropdownItem>
        {#if post.publisher === ipfs_id}
          <DropdownItem
            on:click={() => {
              removePostFromDbAndFeed(post.cid);
            }}>Delete post</DropdownItem
          >
        {:else}
          <DropdownItem
            on:click={() => {
              unfollowPublisher(post.publisher);
            }}>Unfollow publisher</DropdownItem
          >
        {/if}
      </Dropdown>
    {/if}
  </div>

  <div class="flex flex-col">
    <a href="/identity/{post.publisher}">
      {#if post.display_name}
        {post.display_name}
      {:else}
        {post.publisher.slice(0, 32)}...
      {/if}
    </a>
    <div>
      <TimeagoComponent timestamp={post.timestamp} />
    </div>
  </div>

  <br />
  {#if post.body}
    {@html bodyHTML}
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
  {#each media as mediaObj, idx (mediaObj.filename)}
    {#if mediaObj.content_type.includes("image")}
      {#if mediaObj.thumbnail_for}
        <Button
          bind:this={mediaObj.element}
          on:click={() => loadVideo(mediaObj.filename, idx)}
        >
          <PlaySolid />
        </Button>
      {:else}
        <button type="button" on:click={() => openMediaModal(idx)}>
          <img alt="" src={mediaObj.url} />
        </button>
      {/if}
    {:else if mediaObj.content_type.includes("audio")}
      <audio bind:this={mediaObj.element} controls src={mediaObj.url} />
    {:else if mediaObj.content_type.includes("video")}
      <video bind:this={mediaObj.element} controls src={mediaObj.url}>
        <track kind="captions" />
      </video>
    {:else if mediaObj.content_type.includes("pdf")}
      <Button
        download={mediaObj.filename}
        href={mediaObj.url}
        icon={DownloadSolid}
        iconDescription="DownloadSolid"
        kind="secondary"
        on:click={() => saveMedia(mediaObj.filename)}
      >
        {mediaObj.filename}
      </Button>
    {/if}
  {/each}

  <br />

  <ButtonGroup>
    <Button disabled={true}>
      {#if true}
        <ThumbsUpOutline />
      {:else}
        <ThumbsUpSolid />
      {/if}
    </Button>

    <Button disabled={true}>
      {#if true}
        <ThumbsDownOutline />
      {:else}
        <ThumbsDownSolid />
      {/if}
    </Button>

    <Button on:click={() => (replying = !replying)}>
      {#if replying}
        <ReplySolid />
      {:else}
        <ReplyOutline />
      {/if}
    </Button>
  </ButtonGroup>

  {#if replying}
    <Textarea bind:value={reply} placeholder="Add a reply..." />
    <ButtonGroup>
      <Button on:click={postReply}>ReplySolid</Button>
      <Button on:click={cancelReply}>Cancel</Button>
    </ButtonGroup>
  {/if}

  <br />

  {#if show_comments}
    {#each comments as comment (comment.sequenceNumber)}
      <CommentComponent {comment} />
    {/each}
  {/if}
</Card>

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
