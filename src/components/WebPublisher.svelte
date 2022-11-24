<script lang="ts">
  import {
    Button,
    ClickableTile,
    Form,
    FormGroup,
    TextArea,
    TextInput,
    Link,
  } from "carbon-components-svelte";
  // import { format as formatTime } from "timeago.js";
  import WebFeedEntriesComponent from "./WebFeedEntries.svelte";
  import type { WebFeed, WebFeedEntry } from "../types";
  import { invoke } from "@tauri-apps/api";
  import { onMount, onDestroy } from "svelte";

  export let params: object;
  $: publisher = params["b64_publisher"];

  // $: timeago = formatTime(feed ? feed["timestamp"] : 0);
  // $: datetime = new Date(feed ? feed["timestamp"] : 0).toLocaleString();

  let feed: WebFeed;
  let entries: WebFeedEntry[] = [];
  let links: string[] = [];

  onMount(async () => {
    console.log("WebFeedIdentity.onMount");
    console.log(feed);
    console.log(params);
    console.log(publisher);
    feed = await invoke("fetch_webfeed", {
      url: atob(publisher),
    });
    console.log(feed);
    entries = feed.entries;
    links = feed.links;
  });

  onDestroy(() => {});
</script>

{#if feed}
  <Form on:submit>
    <!-- <FormGroup legendText="avatar">
          <UserProfile20>
            <title>Avitar</title>
          </UserProfile20>
          {feed["avatar"]}
        </FormGroup> -->

    {#if feed.title}
      <FormGroup>
        {feed.title}
      </FormGroup>
    {/if}

    {#if feed.description}
      <FormGroup>
        {@html feed.description}
      </FormGroup>
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

    <!-- <FormGroup legendText="Last published">
      {timeago} ({datetime})
    </FormGroup> -->
  </Form>
{/if}
