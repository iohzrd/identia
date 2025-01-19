<script lang="ts">
  import { Button, TextInput } from "carbon-components-svelte";
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  let main: any = $state();
  let height: number = 1000;
  console.log("clientHeight", height);

  let url = $state(
    "https://apnews.com/article/israel-palestinians-hamas-war-lebanon-hezbollah-10-10-2024-a5e53cdd4fca30be1909506b5c777922"
  );
  let htmlString: string = "";
  let blob: Blob;
  let blobUrl: string = $state("");
  const urlCreator = window.URL || window.webkitURL;

  async function fetchUrl() {
    htmlString = await invoke("archive_webpage", { url: url });
    blob = new Blob([htmlString], { type: "text/html" });
    // TODO save safeHtml in sqlite...
    blobUrl = urlCreator.createObjectURL(blob);
  }

  onMount(async () => {
    // height = main.clientHeight;
  });

  onDestroy(() => {});
</script>

<div>
  <TextInput bind:value={url} labelText="url" placeholder="Enter a URL" />
  <Button
    iconDescription="Download"
    kind="secondary"
    on:click={() => fetchUrl()}
  >
    fetch
  </Button>
  {#if blobUrl}
    <div style="background: black;">
      <br />
      <h5>
        Archived from: <a href={url} target="_blank">{url}</a>
      </h5>
      <h6>
        {new Date()}
      </h6>
      <br />
    </div>
    <main bind:this={main} style="background: white;">
      <iframe
        title={url}
        src={blobUrl}
        style="width: 100%; height: {height}px; overflow: auto;"
      >
      </iframe>
    </main>
  {/if}
</div>
