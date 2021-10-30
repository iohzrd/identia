<script lang="ts">
  import type { PostResponse, MediaResponse, MimeRequest } from "../types.type";
  import { Buffer } from "buffer/index";
  import { ClickableTile, Tile, Loading, Link } from "carbon-components-svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";
  import { create } from "ipfs-http-client/index";

  export let cid: String;
  export let postResponse: PostResponse;
  export let includeFrom: Boolean = true;
  let media = [];

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
    const mediaObj = {
      blobUrl: urlCreator.createObjectURL(blob),
      mime: mime,
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
    {#if postResponse.post.files.length > 0 && media.length == 0}
      <Loading withOverlay={false} small />loading media...
    {:else if postResponse.post.files.length > 0 && media.length > 0}
      {#each media as mediaObj}
        {#if mediaObj.mime && mediaObj.mime.includes("image")}
          <img src={mediaObj.blobUrl} alt="" />
        {:else if mediaObj.mime && mediaObj.mime.includes("video")}
          <video src={mediaObj.blobUrl} height="360" controls>
            <track kind="captions" />
          </video>
        {:else if mediaObj.mime && mediaObj.mime.includes("audio")}
          <video src={mediaObj.blobUrl} height="360" controls>
            <track kind="captions" />
          </video>
        {/if}
      {/each}
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
