<script lang="ts">
  import TopicPostComponent from "$lib/TopicPost.svelte";
  import type { MessageExtended } from "$lib/types";
  import type { PageData } from "./$types";
  import { Button, TextArea } from "carbon-components-svelte";
  import { createJson, createTopical } from "$lib/flatbuffers";
  import { ipfs } from "$lib/core";
  import { onMount, onDestroy } from "svelte";
  import { pubsubStore, globalPubsubHandler } from "$lib/pubsub";

  export let data: PageData;

  let unsubscribe: any;
  let posts: MessageExtended[] = [];
  let body: string = "";

  async function post() {
    console.log("post");
    ipfs.pubsub.publish(
      data.topic,
      createJson({ inReplyTo: "root", body: body })
    );
    body = "";
  }

  function messageHandler(message: MessageExtended) {
    console.log("topic.messageHandler", message);
    posts = [message, ...posts];
  }

  onMount(async () => {
    console.log("TopicFeed.onMount");
    const activeSubs = await ipfs.pubsub.ls();
    if (!activeSubs.includes(data.topic)) {
      await ipfs.pubsub.subscribe(data.topic, globalPubsubHandler);
    }
    unsubscribe = pubsubStore.subscribe(data.topic, "root", messageHandler);
  });

  onDestroy(async () => {
    console.log("TopicFeed.onDestroy");
    const activeSubs = await ipfs.pubsub.ls();
    if (activeSubs.includes(data.topic)) {
      await ipfs.pubsub.unsubscribe(data.topic, globalPubsubHandler);
    }
    if (unsubscribe != undefined) {
      unsubscribe();
    }
  });
</script>

<TextArea
  bind:value={body}
  labelText="/{data.topic}/"
  placeholder="What's happening?"
/>
<Button on:click={post}>Post</Button>

{#each posts as post (post.sequenceNumber)}
  <TopicPostComponent {post} />
{/each}
