<script lang="ts">
  // import PdfViewer from "svelte-pdf";
  import type { Media } from "$lib/types";
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
      {#each media_modal_media as mediaObj (mediaObj.filename)}
        <SplideSlide>
          {#if mediaObj.content_type && mediaObj.content_type.includes("image")}
            <img src={mediaObj.url} alt="" bind:this={mediaObj.element} />
          {:else if mediaObj.content_type && mediaObj.content_type.includes("pdf")}
            <!-- <PdfViewer url={mediaObj.url} scale={1.0} showBorder={false} /> -->
          {/if}
        </SplideSlide>
      {/each}
    </Splide>
  {/if}
</Modal>

<style>
  img {
    height: auto;
    width: 100%;
  }

  /* img {
    height: auto;
    width: 100%;
    object-fit: contain;
  } */

  /* @media only screen and (orientation: landscape) {
    img {
      height: auto;
      max-width: 100%;
      object-fit: contain;
    }
  } */
</style>
