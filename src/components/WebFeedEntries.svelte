<script lang="ts">
  import GenericEntryComponent from "./WebFeedEntries/GenericEntry.svelte";
  import YoutubeEntryComponent from "./WebFeedEntries/YoutubeEntry.svelte";
  import type { WebFeedEntry, Media } from "../types";
  import {
    Link,
    OverflowMenu,
    OverflowMenuItem,
    ProgressBar,
    Tile,
  } from "carbon-components-svelte";
  import { format as formatTime } from "timeago.js";
  import { onMount, onDestroy } from "svelte";

  export let media_modal_idx: number;
  export let media_modal_media: Media[];
  export let media_modal_open: boolean;

  export let removePostFromFeed: Function;
  export let ipfs_id: string;
  export let entry: WebFeedEntry;

  let deleting: boolean = false;
  let timeout;
  let timeout_time = 1000;
  let timeago: string = formatTime(entry.timestamp);
  let datetime: string = new Date(entry.timestamp).toLocaleString();

  let repost;

  function newTimeout() {
    timeago = formatTime(entry.timestamp);
    let delta = new Date().getTime() - entry.timestamp;
    if (delta < 60 * 1000) {
      // less than a minue, update once a second
      timeout_time = 1000;
    } else if (delta < 60 * 60 * 1000) {
      // less than an hour, update once a minute
      timeout_time = 60 * 1000;
    } else {
      // update once an hour
      timeout_time = 60 * 60 * 1000;
    }
    timeout = setTimeout(newTimeout, timeout_time);
  }

  onMount(async () => {
    timeout = setTimeout(newTimeout, timeout_time);
  });

  onDestroy(() => {
    clearTimeout(timeout);
  });
</script>

<Tile style="outline: 2px solid black">
  <div>
    <OverflowMenu flipped style="float:right;">
      <OverflowMenuItem
        text="Re-post to identia"
        on:click={() => {
          repost(entry);
        }}
      />
    </OverflowMenu>

    {#if deleting}
      <ProgressBar helperText="Deleting..." />
    {:else}
      <Link size="lg" target="_blank" href={entry.publisher_url}>
        {entry.publisher}
      </Link>
      - {timeago} ({datetime})
      <br />
      <Link size="lg" target="_blank" href={entry.publisher_url}>
        {entry.title}
      </Link>
    {/if}
  </div>
  {#if entry.cid.startsWith("yt:video:")}
    <YoutubeEntryComponent {entry} />
  {:else}
    <GenericEntryComponent {entry} />
  {/if}
</Tile>
