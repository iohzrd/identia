<script lang="ts">
  import {
    Link,
    OverflowMenu,
    OverflowMenuItem,
    ProgressBar,
    Tile,
  } from "carbon-components-svelte";
  import GenericEntryComponent from "./WebFeedEntries/GenericEntry.svelte";
  import OdyseeEntryComponent from "./WebFeedEntries/OdyseeEntry.svelte";
  import YoutubeEntryComponent from "./WebFeedEntries/YoutubeEntry.svelte";
  import type { WebFeedEntry } from "../types";
  import { onMount, onDestroy } from "svelte";

  export let entry: WebFeedEntry;
  // export let removePostFromFeed: Function;

  let deleting: boolean = false;

  function repost(entry: any) {
    console.log("WebFeed repost not yet implemented");
  }

  onMount(async () => {});
  onDestroy(() => {});
</script>

<Tile style="outline: 2px solid black">
  {#if deleting}
    <ProgressBar helperText="Deleting..." />
  {:else}
    <OverflowMenu flipped style="float:right;">
      <OverflowMenuItem
        text="Re-post to identia"
        on:click={() => {
          repost(entry);
        }}
      />
    </OverflowMenu>
  {/if}

  {#if entry.cid.startsWith("yt:video:")}
    <YoutubeEntryComponent {entry} />
  {:else if entry.cid.includes("odysee.com/")}
    <OdyseeEntryComponent {entry} />
  {:else}
    <GenericEntryComponent {entry} />
  {/if}
</Tile>
