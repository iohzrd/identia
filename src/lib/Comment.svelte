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
  import { execute, select } from "./db";
  import { ipfs } from "$lib/core";
  import { onMount, onDestroy } from "svelte";

  export let comment: MessageType;

  let sub_comments: MessageType[] = [];
  let replying = false;
  let reply: string = "";

  export async function insertCommentDB(): Promise<QueryResult> {
    console.log("insertCommentDB");
    await execute("INSERT INTO comments (inReplyTo) VALUES ($1)", [
      "QmY2rrjTX8SeiFDyDzjrbn1yGUw2A4XS6QHhhSPob8So4Q",
    ]);
    return await execute("INSERT INTO comments (inReplyTo) VALUES ($1)", [
      1672812339301452340,
    ]);
  }

  async function postReply() {
    console.log("postReply");
    console.log(reply);
    let r = {
      body: reply,
      inReplyTo: String(comment.sequenceNumber),
      timestamp: new Date().getTime(),
    };
    console.log(r);
    await ipfs.pubsub.publish(
      comment.topic,
      new TextEncoder().encode(JSON.stringify(r))
    );
    reply = "";
    replying = false;
  }

  function cancelReply() {
    reply = "";
    replying = false;
  }

  export async function messageHandler(message: Any) {
    console.log("Comment.messageHandler", message);
    let parsed = JSON.parse(new TextDecoder().decode(message.data));
    console.log(parsed);
    message.inReplyTo = parsed["inReplyTo"];
    let timestamp = Number(String(message.sequenceNumber).slice(0, -6));
    console.log(timestamp);
    message.timestamp = timestamp;
    console.log(message);
    console.log("HERE!!!!");
    console.log(message);
    console.log(comment);

    if (message.inReplyTo === String(comment.sequenceNumber)) {
      sub_comments = [...sub_comments, message];
    }
    // sub_comments = [...sub_comments, message];

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
    console.log("Comment.onMount: ", comment.topic, comment.inReplyTo);
    ipfs.pubsub.subscribe(comment.topic, messageHandler);
    console.log("SELECTING");
    console.log(
      await select("SELECT * FROM comments WHERE inReplyTo = ?", [
        comment.inReplyTo,
      ])
    );
    // sub_comments = await select("SELECT * FROM comments WHERE inReplyTo = ?", [
    //   inReplyTo,
    // ]);
  });

  onDestroy(() => {
    console.log("Comment.onDestroy");
    ipfs.pubsub.unsubscribe(comment.topic, messageHandler);
  });
</script>

<Tile style="outline: 1px solid black">
  {comment.from}
  <br />
  <!-- {message.data} -->
  {JSON.parse(new TextDecoder().decode(comment.data))["body"]}
  <br />
  <br />

  {#if true}
    <Button
      disabled
      icon={ThumbsUp}
      iconDescription="Like"
      kind="ghost"
      size="small"
    />
  {:else}
    <Button
      disabled
      icon={ThumbsUpFilled}
      iconDescription="Like"
      kind="ghost"
      size="small"
    />
  {/if}

  {#if true}
    <Button
      disabled
      icon={ThumbsDown}
      iconDescription="Dislike"
      kind="ghost"
      size="small"
    />
  {:else}
    <Button
      disabled
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

  {#each sub_comments as sub_comment}
    <CommentComponent comment={sub_comment} />
  {/each}
</Tile>
