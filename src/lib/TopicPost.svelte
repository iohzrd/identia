<script lang="ts">
  import Reply from "carbon-icons-svelte/lib/Reply.svelte";
  import TopicPostComponent from "$lib/TopicPost.svelte";
  import type { MessageType } from "$lib/types";
  import { Button, TextArea, Tile } from "carbon-components-svelte";
  import { Comment } from "./flatbuffers/messages_generated";
  import { createComment, pubsubHandler } from "$lib/pubsub";
  import { flatbuffers } from "flatbuffers/js/flatbuffers";
  import { ipfs } from "$lib/core";
  import { onMount, onDestroy } from "svelte";

  export let post: MessageType;

  let unsubscribe: any;

  let sub_replies: any[] = [];
  let replying = false;
  let reply_body: string = "";
  let body: string = "";

  async function postReply() {
    console.log("TopicPost.postReply");
    await ipfs.pubsub.publish(
      post.topic,
      createComment(String(post.sequenceNumber), reply_body)
    );
    reply_body = "";
    replying = false;
  }

  function cancelReply() {
    reply_body = "";
    replying = false;
  }

  async function messageHandler(message: MessageType) {
    sub_replies = [message, ...sub_replies];
  }

  onMount(async () => {
    unsubscribe = pubsubHandler.subscribe(
      post.topic,
      String(post.sequenceNumber),
      messageHandler
    );

    const buff = new flatbuffers.ByteBuffer(post.data);
    const c = Comment.getRootAsComment(buff);
    body = c.body() || "";
  });

  onDestroy(async () => {
    unsubscribe();
  });
</script>

<Tile style="outline: 1px solid black">
  {post.from}
  <br />
  <br />
  {body}
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
