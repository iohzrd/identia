<script lang="ts">
  import TopicPostComponent from "$lib/TopicPost.svelte";
  import type { MessageExtended } from "$lib/types";
  import type { PageData } from "./$types";
  import { Button, TextArea, Tile } from "carbon-components-svelte";
  import { ipfs, log } from "$lib/core";
  import { onMount, onDestroy } from "svelte";
  import { pubsubStore, globalPubsubHandler } from "$lib/pubsub";

  interface Props {
    data: PageData;
  }

  let { data }: Props = $props();

  let unsubscribe: any;
  let posts: MessageExtended[] = $state([]);
  let body: string = $state("");

  async function post() {
    ipfs.pubsub.publish(
      data.topic,
      new TextEncoder().encode(
        JSON.stringify({
          inReplyTo: "root",
          body: body,
        })
      )
    );
    body = "";
  }

  function messageHandler(message: MessageExtended) {
    posts = [message, ...posts];
  }

  onMount(async () => {
    const activeSubs = await ipfs.pubsub.ls();
    if (!activeSubs.includes(data.topic)) {
      await ipfs.pubsub.subscribe(data.topic, globalPubsubHandler);
    }
    unsubscribe = pubsubStore.subscribe(data.topic, "root", messageHandler);
  });

  onDestroy(async () => {
    const activeSubs = await ipfs.pubsub.ls();
    if (activeSubs.includes(data.topic)) {
      await ipfs.pubsub.unsubscribe(data.topic, globalPubsubHandler);
    }
    if (unsubscribe != undefined) {
      unsubscribe();
    }
  });
</script>

<Tile style="outline: 2px solid black">
  <TextArea
    bind:value={body}
    labelText="/{data.topic}/"
    placeholder="What's happening?"
  />
  <Button size="small" on:click={post}>Post</Button>
</Tile>

{#each posts as post (post.sequenceNumber)}
  <TopicPostComponent {post} />
{/each}
