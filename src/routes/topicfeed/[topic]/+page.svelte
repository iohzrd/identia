<script lang="ts">
  import TopicPostComponent from "$lib/TopicPost.svelte";
  import type { MessageExtended } from "$lib/types";
  import type { PageData } from "./$types";
  import { Button, TextArea } from "carbon-components-svelte";
  import { createJson, createTopical } from "$lib/flatbuffers";
  import { ipfs } from "$lib/core";
  import { onMount, onDestroy } from "svelte";
  import { pubsubHandler } from "$lib/pubsub";

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

  onMount(() => {
    console.log("TopicFeed.onMount");
    unsubscribe = pubsubHandler.subscribe(data.topic, "root", messageHandler);
  });

  onDestroy(() => {
    console.log("TopicFeed.onDestroy");
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
