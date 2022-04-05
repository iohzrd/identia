<script lang="ts">
  // import PdfViewer from "svelte-pdf";
  import type { Media } from "../types.type";
  import { Modal } from "carbon-components-svelte";
  import { Splide, SplideSlide } from "@splidejs/svelte-splide";

  export let media_modal_idx: number;
  export let media_modal_media: Media[];
  export let media_modal_open: boolean;
  $: filename =
    typeof Array.isArray(media_modal_media) && media_modal_media.length > 0
      ? media_modal_media[media_modal_idx].filename
      : "";
</script>

<Modal
  bind:open={media_modal_open}
  modalHeading={filename}
  on:close
  on:open
  passiveModal
  size="lg"
>
  {#if media_modal_open}
    <Splide
      options={{ start: media_modal_idx }}
      on:move={(e) => (media_modal_idx = e.detail.index)}
    >
      {#each media_modal_media as mediaObj}
        {#if mediaObj.mime && mediaObj.mime.includes("image")}
          <SplideSlide>
            <img
              class="image"
              src={mediaObj.url}
              alt=""
              bind:this={mediaObj.element}
            />
          </SplideSlide>
        {:else if mediaObj.mime && mediaObj.mime.includes("audio")}
          <SplideSlide>
            <video src={mediaObj.url} controls bind:this={mediaObj.element}>
              <track kind="captions" />
            </video>
          </SplideSlide>
        {:else if mediaObj.mime && mediaObj.mime.includes("video")}
          <SplideSlide>
            <video src={mediaObj.url} controls bind:this={mediaObj.element}>
              <track kind="captions" />
            </video>
          </SplideSlide>
        {:else if mediaObj.mime && mediaObj.mime.includes("pdf")}
          <SplideSlide>
            <div style="text-align: center;">
              PDF
              <!-- <PdfViewer url={mediaObj.url} scale={1.0} showBorder={false} /> -->
            </div>
          </SplideSlide>
        {/if}
      {/each}
    </Splide>
  {/if}
</Modal>

<style>
  .image {
    display: block;
    margin-left: auto;
    margin-right: auto;
    margin: auto;
    max-height: 100%;
    max-width: 100%;
    object-fit: contain;
  }
</style>
