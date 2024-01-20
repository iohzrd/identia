<script lang="ts">
  import type { Media } from "$lib/types";
  import { Modal } from "flowbite-svelte";
  import { Carousel } from "flowbite-svelte";
  import { Thumbnails } from "flowbite-svelte";

  import type { HTMLImgAttributes } from "svelte/elements";
  import { onMount, onDestroy } from "svelte";

  export let media: Media[];
  export let open: boolean;
  let images: HTMLImgAttributes[] = [];
  let index = 0;
  let forward = true; // sync animation direction between Thumbnails and Carousel
  // $: filename =
  //   typeof Array.isArray(media) && media.length > 0
  //     ? media[start].filename
  //     : "";

  function map_images(m: Media): HTMLImgAttributes {
    return {
      alt: m.filename,
      src: m.url,
      title: m.filename,
    };
  }

  onMount(async () => {
    console.log("MediaModal.onMount");
    console.log(open);
    images = media.filter((m) => m.filename != "post.json").map(map_images);
    console.log("images");
    console.log(images);
  });

  onDestroy(() => {});
</script>

{#if open}
  <Modal bind:open autoclose size="xl">
    <div class="max-w-4xl space-y-4">
      <Carousel
        {images}
        {forward}
        let:Indicators
        let:Controls
        bind:index
        transition={null}
      >
        <Controls />
        <Indicators />
      </Carousel>
      <Thumbnails {images} {forward} bind:index />
    </div>
  </Modal>
{/if}
