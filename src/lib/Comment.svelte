<script lang="ts">
  import CommentComponent from "$lib/Comment.svelte";
  import Reply from "carbon-icons-svelte/lib/Reply.svelte";
  import ThumbsDown from "carbon-icons-svelte/lib/ThumbsDown.svelte";
  import ThumbsDownFilledfrom from "carbon-icons-svelte/lib/ThumbsDownFilled.svelte";
  import ThumbsUp from "carbon-icons-svelte/lib/ThumbsUp.svelte";
  import ThumbsUpFilled from "carbon-icons-svelte/lib/ThumbsUpFilled.svelte";
  import type { MessageType } from "$lib/types";
  import type { QueryResult } from "tauri-plugin-sql-api";
  import { Button, TextArea, Tile } from "carbon-components-svelte";
  import { Comment } from "./flatbuffers/messages_generated";
  import { createComment, pubsubHandler } from "$lib/pubsub";
  import { execute, select } from "./db";
  import { flatbuffers } from "flatbuffers/js/flatbuffers";
  import { ipfs } from "$lib/core";
  import { onMount, onDestroy } from "svelte";

  export let comment: MessageType;

  let sub_comments: Any[] = [];
  let replying = false;
  let reply: string = "";
  let body: string = "";

  async function postReply() {
    console.log("Comment.postReply");
    console.log(comment);
    // let r = {
    //   body: reply,
    //   inReplyTo: String(comment.sequenceNumber),
    //   timestamp: new Date().getTime(),
    // };
    // console.log(r);
    await ipfs.pubsub.publish(
      comment.topic,
      createComment(String(comment.sequenceNumber), reply)

      // new TextEncoder().encode(JSON.stringify(r))
    );
    reply = "";
    replying = false;
  }

  function cancelReply() {
    reply = "";
    replying = false;
  }

  async function messageHandler(message: MessageType) {
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

  const unsubscribe = pubsubHandler.subscribe(
    comment.topic,
    String(comment.sequenceNumber),
    messageHandler
  );

  onMount(async () => {
    // await ipfs.pubsub.subscribe(comment.topic, messageHandler);

    // let parsed = JSON.parse(new TextDecoder().decode(comment.data));
    // body = parsed["body"];

    const buff = new flatbuffers.ByteBuffer(comment.data);
    const c = Comment.getRootAsComment(buff);
    body = c.body() || "";

    // sub_comments = await select("SELECT * FROM comments WHERE inReplyTo = ?", [
    //   inReplyTo,
    // ]);
  });

  onDestroy(async () => {
    // await ipfs.pubsub.unsubscribe(comment.topic, messageHandler);
    unsubscribe;
  });
</script>

<Tile style="outline: 1px solid black">
  {comment.from}
  <br />
  <br />
  {body}
  <br />
  <br />

  {#if true}
    <Button icon={ThumbsUp} iconDescription="Like" kind="ghost" size="small" />
  {:else}
    <Button
      icon={ThumbsUpFilled}
      iconDescription="Like"
      kind="ghost"
      size="small"
    />
  {/if}

  {#if true}
    <Button
      icon={ThumbsDown}
      iconDescription="Dislike"
      kind="ghost"
      size="small"
    />
  {:else}
    <Button
      icon={ThumbsDownFilledfrom}
      iconDescription="Dislike"
      kind="ghost"
      size="small"
    />
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
