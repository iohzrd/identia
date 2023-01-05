<script lang="ts">
  import CommentComponent from "$lib/Comment.svelte";
  import Reply from "carbon-icons-svelte/lib/Reply.svelte";
  import ThumbsDown from "carbon-icons-svelte/lib/ThumbsDown.svelte";
  import ThumbsDownFilledfrom from "carbon-icons-svelte/lib/ThumbsDownFilled.svelte";
  import ThumbsUp from "carbon-icons-svelte/lib/ThumbsUp.svelte";
  import ThumbsUpFilled from "carbon-icons-svelte/lib/ThumbsUpFilled.svelte";
  import type { CommentType } from "$lib/types";
  import type { Message } from "ipfs-http-client/pubsub/subscribe";
  import type { QueryResult } from "tauri-plugin-sql-api";
  import { Button, TextArea, Tile } from "carbon-components-svelte";
  import { execute, select } from "./db";
  import { ipfs } from "$lib/core";
  import { onMount, onDestroy } from "svelte";
  import { publish } from "$lib/pubsub";

  export let topic: string;
  export let inReplyTo: string | bigint;
  let comment = { comment: "Comment...", from: "someone" };
  let sub_comments = [];
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
    let message: CommentType = {
      body: reply,
      inReplyTo: String(inReplyTo),
      timestamp: new Date().getTime(),
    };
    await publish(topic, new TextEncoder().encode(JSON.stringify(message)));
    reply = "";
    replying = false;
  }

  function cancelReply() {
    reply = "";
    replying = false;
  }

  export async function messageHandler(message: Message) {
    console.log("messageHandler", message);
    console.log(JSON.parse(new TextDecoder().decode(message.data)));
    // 1672809820178706035n
    // 1672809820178
  }

  onMount(async () => {
    console.log("Comment.onMount: ", topic, inReplyTo);
    ipfs.pubsub.subscribe(topic, messageHandler);
    await insertCommentDB();
    console.log(await select("SELECT * FROM comments"));
    // comment = await select("SELECT * FROM comments", [inReplyTo]);
    // sub_comments = await select("SELECT * FROM comments WHERE inReplyTo = ?", [
    //   inReplyTo,
    // ]);
  });

  onDestroy(() => {
    console.log("Comment.onDestroy");
    ipfs.pubsub.unsubscribe(topic, messageHandler);
  });
</script>

<Tile style="outline: 1px solid black">
  {comment.from}
  <br />
  {comment.comment}
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

  {#each sub_comments as comment}
    <CommentComponent {topic} inReplyTo={comment.inReplyTo} />
  {/each}
</Tile>
