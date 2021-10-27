<script lang="ts">
  import type { PostResponse, MediaResponse, MimeRequest } from "../types.type";
  import { Buffer } from "buffer/index";
  import { ClickableTile } from "carbon-components-svelte";
  import { Link } from "carbon-components-svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";
  import { create } from "ipfs-http-client/index";
  // import VideoPlayer from "svelte-video-player";
  // import { fromBuffer } from "file-type";

  export let cid: String;
  export let postResponse: PostResponse;
  export let includeFrom: Boolean = true;
  let blobs = [];

  async function getPostFromCid() {
    console.log("getPostFromCid");
    postResponse = await invoke("get_post_ipfs", {
      cid: cid,
    });
  }

  // // this version is much slower and less efficient
  // async function getFile(filename) {
  //   const root_cid = postResponse.cid || cid;
  //   const path = root_cid + "/" + filename;
  //   const mr: MediaResponse = await invoke("get_file_ipfs", {
  //     cid: path,
  //   });
  //   const buf = Buffer.from(mr.data);
  //   const blob = new Blob([buf], { type: mr.mime });
  //   const urlCreator = window.URL || window.webkitURL;
  //   const mediaObj = {
  //     blob: urlCreator.createObjectURL(blob),
  //     mime: mr.mime,
  //   };
  //   blobs = [...blobs, mediaObj];
  //   return mediaObj;
  // }

  // this version is more efficient but,
  // currently doesn't work because webkit2gtk CORS bug...
  // fixed in webkit2gtk 2.34.0
  async function getFile(filename) {
    console.log("getFile");
    let bufs = [];
    let mime: string;
    const root_cid = postResponse.cid || cid;
    const path = root_cid + "/" + filename;
    const ipfs = await create("/ip4/127.0.0.1/tcp/5001");
    for await (const buf of ipfs.cat(path)) {
      bufs.push(buf);
    }
    const buf = Buffer.concat(bufs);

    // this is sub-optimal but is required because
    // file-type doesn't work in svelte currently
    mime = await invoke("get_mime", {
      buf: buf.slice(0, 16),
    });
    // mime = (await fromBuffer(buf)).mime;

    const blob = new Blob([buf], { type: mime });
    const urlCreator = window.URL || window.webkitURL;
    const mediaObj = {
      blob: urlCreator.createObjectURL(blob),
      mime: mime,
    };
    blobs = [...blobs, mediaObj];
    return mediaObj;
  }

  onMount(async () => {
    if (!postResponse) {
      await getPostFromCid();
    }

    // postResponse.post.files.forEach((filename) => {
    //   getFile(filename);
    // });

    for await (const filename of postResponse.post.files) {
      // blobs = [...blobs, await getFile(filename)];
      await getFile(filename);
      // getFile(filename);
    }
  });

  onDestroy(() => {});
</script>

<ClickableTile>
  {#if postResponse.post}
    {#if postResponse.post.body}
      <div>
        {@html postResponse.post.body.replace(/\n/g, "<br>")}
        <!-- {post.body} -->
      </div>
    {/if}
    {#if blobs}
      {#each blobs as media}
        {#if media.mime && media.mime.includes("image")}
          <img src={media.blob} alt="" />
        {:else if media.mime && media.mime.includes("video")}
          <video src={media.blob} width="640" height="480" controls />
          <!-- <VideoPlayer source={media.blob} /> -->
        {/if}
      {/each}
    {:else if postResponse.post.files}
      loading files...
    {/if}
    {#if postResponse.post.publisher && includeFrom}
      <div>
        publisher: <Link href="#/identity/{postResponse.post.publisher}">
          {postResponse.post.publisher}
        </Link>
      </div>
    {/if}
    {#if postResponse.post.timestamp}
      <div>
        {postResponse.post.timestamp}
      </div>
    {/if}
  {/if}
</ClickableTile>
