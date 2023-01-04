<script lang="ts">
  import CommentComponent from "$lib/Comment.svelte";
  import Reply from "carbon-icons-svelte/lib/Reply.svelte";
  import ThumbsDown from "carbon-icons-svelte/lib/ThumbsDown.svelte";
  import ThumbsDownFilledfrom from "carbon-icons-svelte/lib/ThumbsDownFilled.svelte";
  import ThumbsUp from "carbon-icons-svelte/lib/ThumbsUp.svelte";
  import ThumbsUpFilled from "carbon-icons-svelte/lib/ThumbsUpFilled.svelte";
  import { Button, TextArea, Tile } from "carbon-components-svelte";
  import { publish } from "$lib/pubsub";
  import { onMount, onDestroy } from "svelte";
  import { select } from "./db";

  export let topic: string;
  export let inReplyTo: string | bigint;
  let comment = { comment: "Comment...", from: "someone" };
  let sub_comments = [];
  let replying = false;
  let reply: string = "";

  async function postReply() {
    console.log("postReply");
    console.log(reply);
    // publish(inReplyTo, reply);
    reply = "";
    replying = false;
  }

  function cancelReply() {
    reply = "";
    replying = false;
  }

  onMount(async () => {
    console.log("Comment.onMount: ", topic, inReplyTo);
    comment = await select("SELECT * FROM comments WHERE inReplyTo = ?", [
      inReplyTo,
    ]);
    sub_comments = await select("SELECT * FROM comments WHERE inReplyTo = ?", [
      inReplyTo,
    ]);
  });

  onDestroy(() => {
    console.log("Comment.onDestroy");
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
