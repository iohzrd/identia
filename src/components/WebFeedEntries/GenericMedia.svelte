<script lang="ts">
  import PlayFilled from "carbon-icons-svelte/lib/PlayFilled.svelte";
  import type {
    WebFeedEntry,
    WebFeedMediaContent,
    WebFeedMediaObject,
  } from "../../types";
  // import { fetch, ResponseType } from "@tauri-apps/api/http";
  import { onMount, onDestroy } from "svelte";

  export let entry: WebFeedEntry;

  let media = [];
  let mediaObjects: WebFeedMediaObject[] = entry.media.map((obj) => obj);
  let mediaContent: WebFeedMediaContent[] = mediaObjects.flatMap((m) =>
    m.content.map((c) => c)
  );

  async function getMedia(media, isThumbnail = false) {
    console.log("getMedia");
    let mediaObj = {
      url: media.url,
      element: null,
      thumbnailFor: null,
      content_type: media.content_type,
    };
    if (isThumbnail) {
      mediaObj.thumbnailFor = media.url;
    }
    return mediaObj;
  }

  async function loadMedia(mc, idx: number) {
    console.log("loadMedia: ", idx);
    media[idx] = await getMedia(mc);
    media = media;
  }

  onMount(async () => {
    // let resp = await fetch(link, {
    //   method: "GET",
    //   timeout: 30,
    //   responseType: ResponseType.Text,
    // });
    for await (const mc of mediaContent) {
      media = [...media, await getMedia(mc, true)];
    }
  });

  onDestroy(() => {});
</script>

{#each media as mediaObj, idx (mediaObj.url)}
  {#if mediaObj.thumbnailFor}
    <div
      bind:this={mediaObj.element}
      on:click={() => loadMedia(mediaObj, idx)}
      on:keypress
    >
      <PlayFilled size={32} />
    </div>
  {:else if mediaObj.content_type.includes("audio/")}
    <audio bind:this={mediaObj.element} controls src={mediaObj.url} />
  {:else if mediaObj.content_type.includes("image/")}
    <img bind:this={mediaObj.element} src={mediaObj.url} alt={mediaObj.url} />
  {:else if mediaObj.content_type.includes("video/")}
    <video bind:this={mediaObj.element} controls src={mediaObj.url}>
      <track kind="captions" />
    </video>
  {/if}
{/each}

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
