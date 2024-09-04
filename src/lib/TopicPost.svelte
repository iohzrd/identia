<script lang="ts">
  import Reply from "carbon-icons-svelte/lib/Reply.svelte";
  import TopicPostComponent from "$lib/TopicPost.svelte";
  import type { MessageExtended } from "$lib/types";
  import { Button, TextArea, Tile } from "carbon-components-svelte";
  import { ipfs } from "$lib/core";
  import { onMount, onDestroy } from "svelte";
  import { pubsubStore } from "$lib/pubsub";

  export let post: MessageExtended;

  let unsubscribe: any;

  let sub_replies: any[] = [];
  let replying = false;
  let reply_body: string = "";
  let body: string = "";

  async function postReply() {
    await ipfs.pubsub.publish(
      post.topic,
      new TextEncoder().encode(
        JSON.stringify({
          inReplyTo: String(post.sequenceNumber),
          body: reply_body,
        })
      )
    );
    reply_body = "";
    replying = false;
  }

  function cancelReply() {
    reply_body = "";
    replying = false;
  }

  async function messageHandler(message: MessageExtended) {
    sub_replies = [message, ...sub_replies];
  }

  onMount(async () => {
    unsubscribe = pubsubStore.subscribe(
      post.topic,
      String(post.sequenceNumber),
      messageHandler
    );
  });

  onDestroy(async () => {
    unsubscribe();
  });
</script>

<Tile style="outline: 2px solid black">
  {post.from}
  <br />
  <br />
  {post.body}
  <br />
  <br />

  <Button
    icon={Reply}
    iconDescription="Reply"
    kind="ghost"
    size="small"
    on:click={() => {
      replying = !replying;
    }}
  />

  {#if replying}
    <div>
      <TextArea bind:value={reply_body} placeholder="Add a reply..." rows={2} />
      <Button
        kind="ghost"
        size="small"
        style="float:right;"
        on:click={postReply}>Reply</Button
      >
      <Button
        kind="ghost"
        size="small"
        style="float:right;"
        on:click={cancelReply}>Cancel</Button
      >
    </div>
    <br />
    <br />
  {/if}

  {#each sub_replies as reply (reply.sequenceNumber)}
    <TopicPostComponent post={reply} />
  {/each}
</Tile>
