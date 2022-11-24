<script lang="ts">
  import WebFeedEntriesComponent from "./WebFeedEntries.svelte";
  import type { WebFeed, WebFeedEntry } from "../types";
  import { Form, FormGroup, Link } from "carbon-components-svelte";
  import { invoke } from "@tauri-apps/api";
  import { onMount, onDestroy } from "svelte";

  export let params: object;
  let publisher = atob(params["b64_publisher"]);

  let entries: WebFeedEntry[] = [];
  let feed: WebFeed;
  let links: string[] = [];
  let logo: object | null;
  let published_or_updated: string | null = null;
  let title: string = "";

  onMount(async () => {
    console.log("WebFeedIdentity.onMount");
    feed = await invoke("fetch_webfeed", {
      url: publisher,
    });
    console.log(feed);
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
      <FormGroup legendText="Last published">
        {published_or_updated}
      </FormGroup>
    {/if}

    {#if feed.description}
      <h4>
        <FormGroup legendText="description">
          {@html feed.description}
        </FormGroup>
      </h4>
    {/if}

    {#if links}
      <FormGroup legendText="links">
        {#each links as link (link)}
          <Link size="lg" target="_blank" href={link}>
            {link}
          </Link>
          <br />
        {/each}
      </FormGroup>
    {/if}

    {#if entries}
      <FormGroup legendText="entries">
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
    display: flex;
    vertical-align: middle;
    align-items: center;
  }
</style>
