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
  import type { WebFeedEntry } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";

  export let entry: WebFeedEntry;
  // export let removePostFromFeed: Function;

  let deleting: boolean = false;

  async function repost(entry: WebFeedEntry) {
    await invoke("repost_webfeed_entry", {
      entry: entry,
    });
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

  <Link size="lg" href="/webpublisher/{btoa(entry.publisher)}">
    {entry.display_name}
  </Link>
  -
  {#if entry.cid.startsWith("yt:video:")}
    <YoutubeEntryComponent {entry} />
  {:else if entry.cid.includes("odysee.com/")}
    <OdyseeEntryComponent {entry} />
  {:else}
    <GenericEntryComponent {entry} />
  {/if}
</Tile>
