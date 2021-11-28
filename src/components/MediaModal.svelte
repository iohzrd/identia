<script lang="ts">
  import type { MediaObj } from "../types.type";
  import { Modal } from "carbon-components-svelte";
  import { Splide, SplideSlide } from "@splidejs/svelte-splide";

  export let media_modal_media: MediaObj[];
  export let media_modal_open: boolean;
</script>

{#if media_modal_open}
  <Modal
    bind:open={media_modal_open}
    modalHeading="media"
    on:close
    on:open
    passiveModal={true}
    size="lg"
  >
    <Splide>
      {#each media_modal_media as mediaObj}
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
        {/if}
      {/each}
    </Splide>
  </Modal>
{/if}

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
