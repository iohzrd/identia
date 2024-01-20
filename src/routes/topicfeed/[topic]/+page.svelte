<script lang="ts">
  import TopicPostComponent from "$lib/TopicPost.svelte";
  import type { MessageExtended } from "$lib/types";
  import type { PageData } from "./$types";

  import { Button } from "flowbite-svelte";
  import { Textarea } from "flowbite-svelte";
  import { Card } from "flowbite-svelte";

  import { createJson, createTopical } from "$lib/flatbuffers";
  import { ipfs, log } from "$lib/core";
  import { onMount, onDestroy } from "svelte";
  import { pubsubStore, globalPubsubHandler } from "$lib/pubsub";

  export let data: PageData;

  let unsubscribe: any;
  let posts: MessageExtended[] = [];
  let body: string = "";

  async function post() {
    ipfs.pubsub.publish(
      data.topic,
      createJson({ inReplyTo: "root", body: body })
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

<Card style="outline: 2px solid black">
  <Textarea
    bind:value={body}
    labelText="/{data.topic}/"
    placeholder="What's happening?"
  />
  <Button on:click={post}>Post</Button>
</Card>

{#each posts as post (post.sequenceNumber)}
  <TopicPostComponent {post} />
{/each}
