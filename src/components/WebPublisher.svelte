<script lang="ts">
  import { Form, FormGroup, Link } from "carbon-components-svelte";
  import WebFeedEntriesComponent from "./WebFeedEntries.svelte";
  import type { WebFeed, WebFeedEntry } from "../types";
  import { invoke } from "@tauri-apps/api";
  import { onMount, onDestroy } from "svelte";

  export let params: object;

  let feed: WebFeed;
  let entries: WebFeedEntry[] = [];
  let links: string[] = [];
  let published_or_updated: string | null = null;

  onMount(async () => {
    console.log("WebFeedIdentity.onMount");
    let publisher = atob(params["b64_publisher"]);
    feed = await invoke("fetch_webfeed", {
      url: publisher,
    });
    console.log(feed);
    entries = feed.entries;
    links = feed.links;
    published_or_updated = feed.published || feed.updated;
  });

  onDestroy(() => {});
</script>

{#if feed}
  <Form>
    <!-- <FormGroup legendText="avatar">
          <UserProfile20>
            <title>Avitar</title>
          </UserProfile20>
          {feed["avatar"]}
        </FormGroup> -->

    {#if feed.title}
      <h1>
        <FormGroup>
          {feed.title}
        </FormGroup>
      </h1>
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
