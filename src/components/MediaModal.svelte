<script lang="ts">
  import PdfViewer from "svelte-pdf-viewer";
  import type { MediaObj } from "../types.type";
  import { Modal } from "carbon-components-svelte";
  import { Splide, SplideSlide } from "@splidejs/svelte-splide";

  export let media_modal_media: MediaObj[];
  export let media_modal_open: boolean;
  let pdfInfos = [];
</script>

<Modal
  bind:open={media_modal_open}
  modalHeading="media"
  on:close
  on:open
  passiveModal
  size="lg"
>
  {#if media_modal_open}
    <Splide>
      {#each media_modal_media as mediaObj, idx}
        {#if mediaObj.mime && mediaObj.mime.includes("image")}
          <SplideSlide>
            <img
              class="image"
              src={mediaObj.blobUrl}
              alt=""
              bind:this={mediaObj.element}
            />
          </SplideSlide>
        {:else if mediaObj.mime && mediaObj.mime.includes("audio")}
          <SplideSlide>
            <video src={mediaObj.blobUrl} controls bind:this={mediaObj.element}>
              <track kind="captions" />
            </video>
          </SplideSlide>
        {:else if mediaObj.mime && mediaObj.mime.includes("video")}
          <SplideSlide>
            <video src={mediaObj.blobUrl} controls bind:this={mediaObj.element}>
              <track kind="captions" />
            </video>
          </SplideSlide>
        {:else if mediaObj.mime && mediaObj.mime.includes("pdf")}
          <SplideSlide>
            <div style="height: 90%; width: 100%;">
              <PdfViewer file={mediaObj.blobUrl} bind:infos={pdfInfos[idx]} />
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
