<script lang="ts">
  import WebFeedEntriesComponent from "../../webfeed/WebFeedEntries.svelte";
  import type { PageData } from "./$types";
  import type { WebFeed, WebFeedEntry } from "$lib/types";
  import { Form, FormGroup, Link } from "carbon-components-svelte";
  import { invoke } from "@tauri-apps/api/core";
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
  <Form>
    {#if feed.title}
      <FormGroup>
        <div class="title">
          {#if logo && logo["uri"]}
            <img class="thumbnail" src={logo["uri"]} alt="" />
          {/if}
          <h1>
            {feed.title}
          </h1>
        </div>
      </FormGroup>
    {/if}

    {#if published_or_updated}
      <FormGroup legendText="Updated">
        {published_or_updated}
      </FormGroup>
    {/if}

    {#if feed.description}
      <FormGroup legendText="Description">
        <h4>
          {@html feed.description}
        </h4>
      </FormGroup>
    {/if}

    {#if links}
      <FormGroup legendText="Links">
        {#each links as link (link)}
          <Link size="lg" target="_blank" href={link}>
            {link}
          </Link>
          <br />
        {/each}
      </FormGroup>
    {/if}

    {#if entries}
      <FormGroup legendText="Entries">
        {#each entries as entry (entry.cid)}
          <div>
            <WebFeedEntriesComponent {entry} />
          </div>
        {/each}
      </FormGroup>
    {/if}
  </Form>
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
