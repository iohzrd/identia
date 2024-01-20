<script lang="ts">
  import { Card } from "flowbite-svelte";
  import { Dropdown } from "flowbite-svelte";
  import { DropdownItem } from "flowbite-svelte";
  import { Progressbar } from "flowbite-svelte";

  import GenericEntryComponent from "./WebFeedEntries/GenericEntry.svelte";
  import OdyseeEntryComponent from "./WebFeedEntries/OdyseeEntry.svelte";
  import YoutubeEntryComponent from "./WebFeedEntries/YoutubeEntry.svelte";
  import type { WebFeedEntry } from "$lib/types";
  import { invoke } from "@tauri-apps/api/tauri";
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

<Card style="outline: 2px solid black">
  {#if deleting}
    <Progressbar helperText="Deleting..." />
  {:else}
    <Dropdown flipped style="float:right;">
      <DropdownItem
        text="Re-post to identia"
        on:click={() => {
          repost(entry);
        }}
      />
    </Dropdown>
  {/if}

  <a href="/webpublisher/{btoa(entry.publisher)}">
    {entry.display_name}
  </a>
  -
  {#if entry.cid.startsWith("yt:video:")}
    <YoutubeEntryComponent {entry} />
  {:else if entry.cid.includes("odysee.com/")}
    <OdyseeEntryComponent {entry} />
  {:else}
    <GenericEntryComponent {entry} />
  {/if}
</Card>
