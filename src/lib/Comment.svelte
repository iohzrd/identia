<script lang="ts">
  import CommentComponent from "$lib/Comment.svelte";
  import type { MessageExtended } from "$lib/types";
  import type { QueryResult } from "tauri-plugin-sql-api";
  import { Button } from "flowbite-svelte";
  import { Card } from "flowbite-svelte";
  import { Textarea } from "flowbite-svelte";
  import { ThumbsDownOutline } from "flowbite-svelte-icons";
  import { ThumbsDownSolid } from "flowbite-svelte-icons";
  import { ThumbsUpOutline } from "flowbite-svelte-icons";
  import { ThumbsUpSolid } from "flowbite-svelte-icons";
  import { ReplySolid } from "flowbite-svelte-icons";

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

<Card style="outline: 2px solid black">
  {comment.from}
  <br />
  <br />
  {comment.body}
  <br />
  <br />

  {#if true}
    <Button icon={ThumbsUpOutline} iconDescription="Like" kind="ghost" />
  {:else}
    <Button icon={ThumbsUpSolid} iconDescription="Like" kind="ghost" />
  {/if}

  {#if true}
    <Button icon={ThumbsDownOutline} iconDescription="Dislike" kind="ghost" />
  {:else}
    <Button icon={ThumbsDownSolid} iconDescription="Dislike" kind="ghost" />
  {/if}

  <Button
    icon={ReplySolid}
    iconDescription="Reply"
    kind="ghost"
    on:click={() => {
      replying = !replying;
    }}
  />

  {#if replying}
    <div>
      <Textarea bind:value={reply} placeholder="Add a reply..." rows={2} />
      <Button kind="ghost" style="float:right;" on:click={postReply}
        >Reply</Button
      >
      <Button kind="ghost" style="float:right;" on:click={cancelReply}
        >Cancel</Button
      >
    </div>
    <br />
    <br />
  {/if}

  {#each sub_comments as sub_comment (sub_comment.sequenceNumber)}
    <CommentComponent comment={sub_comment} />
  {/each}
</Card>
