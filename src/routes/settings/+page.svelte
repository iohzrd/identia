<script lang="ts">
  import { onMount } from "svelte";

  let main: any = $state();
  let height: Number = $state(1000);

  onMount(() => {
    height = main.clientHeight;
  });
</script>

<main bind:this={main}>
  <iframe
    title="IPFS webui"
    src="http://webui.ipfs.io.ipns.localhost:8080/"
    frameborder="0"
    width="100%"
    style="width: 100%; height: {height}px; overflow: auto;"
  ></iframe>
</main>
