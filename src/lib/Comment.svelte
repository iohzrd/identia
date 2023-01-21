<script lang="ts">
  import CommentComponent from "$lib/Comment.svelte";
  import Reply from "carbon-icons-svelte/lib/Reply.svelte";
  import type { MessageExtended } from "$lib/types";
  import type { QueryResult } from "tauri-plugin-sql-api";
  import { Button, TextArea, Tile } from "carbon-components-svelte";
  import { ThumbsDown as TD } from "carbon-icons-svelte/lib/";
  import { ThumbsDownFilled as TDF } from "carbon-icons-svelte/lib/";
  import { ThumbsUp as TU } from "carbon-icons-svelte/lib/";
  import { ThumbsUpFilled as TUF } from "carbon-icons-svelte/lib/";
  import { createJson, createTopical } from "$lib/flatbuffers";
  import { execute, select } from "./db";
  import { ipfs } from "$lib/core";
  import { onMount, onDestroy } from "svelte";
  import { pubsubStore } from "$lib/pubsub";

  export let comment: MessageExtended;

  let reply: string = "";
  let replying = false;
  let sub_comments: any[] = [];
  let unsubscribe: any;

  async function postReply() {
    await ipfs.pubsub.publish(
      comment.topic,
      createJson({
        body: reply,
        inReplyTo: String(comment.sequenceNumber),
      })
    );
    reply = "";
    replying = false;
  }

  function cancelReply() {
    reply = "";
    replying = false;
  }

  async function messageHandler(message: MessageExtended) {
    sub_comments = [message, ...sub_comments];

    // await execute(
    //   "INSERT INTO comments (data,from,inReplyTo,key,sequenceNumber,signature,timestamp,topic,type) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)",
    //   [
    //     message.data,
    //     message.from,
    //     message.inReplyTo,
    //     message.key,
    //     String(message.sequenceNumber),
    //     message.signature,
    //     message.timestamp,
    //     message.topic,
    //     message.type,
    //   ]
    // );
  }

  onMount(async () => {
    unsubscribe = pubsubStore.subscribe(
      comment.topic,
      String(comment.sequenceNumber),
      messageHandler
    );
    // sub_comments = await select("SELECT * FROM comments WHERE inReplyTo = ?", [
    //   inReplyTo,
    // ]);
  });

  onDestroy(async () => {
    unsubscribe();
  });
</script>

<Tile style="outline: 2px solid black">
  {comment.from}
  <br />
  <br />
  {comment.body}
  <br />
  <br />

  {#if true}
    <Button icon={TU} iconDescription="Like" kind="ghost" size="small" />
  {:else}
    <Button icon={TUF} iconDescription="Like" kind="ghost" size="small" />
  {/if}

  {#if true}
    <Button icon={TD} iconDescription="Dislike" kind="ghost" size="small" />
  {:else}
    <Button icon={TDF} iconDescription="Dislike" kind="ghost" size="small" />
  {/if}

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
      <TextArea bind:value={reply} placeholder="Add a reply..." rows={2} />
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

  {#each sub_comments as sub_comment (sub_comment.sequenceNumber)}
    <CommentComponent comment={sub_comment} />
  {/each}
</Tile>
