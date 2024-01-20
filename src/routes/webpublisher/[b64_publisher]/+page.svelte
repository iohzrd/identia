<script lang="ts">
  import WebFeedEntriesComponent from "../../webfeed/WebFeedEntries.svelte";
  import type { PageData } from "./$types";
  import type { WebFeed, WebFeedEntry } from "$lib/types";

  import { invoke } from "@tauri-apps/api";
  import { onMount, onDestroy } from "svelte";

  export let data: PageData;
  let publisher = atob(data.b64_publisher);

  let entries: WebFeedEntry[] = [];
  let feed: WebFeed;
  let links: string[] = [];
  let logo: object | null;
  let published_or_updated: string | null = null;
  let title: string = "";

  onMount(async () => {
    feed = await invoke("fetch_webfeed", {
      url: publisher,
    });
    entries = feed.entries;
    links = feed.links
      .map((link) => link.replace("http://", "https://"))
      .filter((link) => link != publisher);
    logo = feed.logo;
    published_or_updated = feed.published || feed.updated;
    title = feed.title;
  });

  onDestroy(() => {});
</script>

{#if feed}
  <div>
    {#if feed.title}
      <div>
        <div class="title">
          {#if logo && logo["uri"]}
            <img class="thumbnail" src={logo["uri"]} alt="" />
          {/if}
          <h1>
            {feed.title}
          </h1>
        </div>
      </div>
    {/if}

    {#if published_or_updated}
      <div legendText="Updated">
        {published_or_updated}
      </div>
    {/if}

    {#if feed.description}
      <div legendText="Description">
        <h4>
          {@html feed.description}
        </h4>
      </div>
    {/if}

    {#if links}
      <div legendText="Links">
        {#each links as link (link)}
          <a target="_blank" href={link}>
            {link}
          </a>
          <br />
        {/each}
      </div>
    {/if}

    {#if entries}
      <div legendText="Entries">
        {#each entries as entry (entry.cid)}
          <div>
            <WebFeedEntriesComponent {entry} />
          </div>
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style>
  .thumbnail {
    border-radius: 50%;
    width: 100px;
  }

  .title {
    align-items: center;
    display: flex;
  }
</style>
