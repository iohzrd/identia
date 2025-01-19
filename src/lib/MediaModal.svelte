<script lang="ts">
  // import PdfViewer from "svelte-pdf";
  import type { Media } from "$lib/types";
  import { Modal } from "carbon-components-svelte";
  import { Splide, SplideSlide } from "@splidejs/svelte-splide";
  import { onMount, onDestroy } from "svelte";

  interface Props {
    start: number;
    media: Media[];
    open: boolean;
  }

  let {
    start = $bindable(),
    media = $bindable(),
    open = $bindable(),
  }: Props = $props();
  let filename = $derived(
    typeof Array.isArray(media) && media.length > 0 ? media[start].filename : ""
  );

  onMount(async () => {
    media = media.filter((m) => m.filename != "post.json");
  });

  onDestroy(() => {});
</script>

<Modal
  bind:open
  modalHeading={filename}
  on:close
  on:open
  passiveModal
  size="lg"
>
  {#if open}
    <Splide
      options={{ start: start }}
      on:move={(e) => (start = e.detail.index)}
    >
      {#each media as mediaObj (mediaObj.filename)}
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
