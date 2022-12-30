<script lang="ts">
  import FileDrop from "svelte-tauri-filedrop";
  import { createTopicPost, parsePubsubMessage } from "../pubsub";
  import { ipfs } from "../core";
  import { onMount, onDestroy } from "svelte";
  import {
    Button,
    FileUploaderItem,
    Form,
    FormGroup,
    ProgressBar,
    TextArea,
    Tile,
  } from "carbon-components-svelte";

  export let params: object;
  let topic: string = params["topic"];
  let posts = [];

  let body: string = "";
  let files: string[] = [];
  let posting = false;

  async function handleFiles() {
    console.log("handleFiles");
  }

  async function removeFile(index) {
    console.log("removeFile", index);
  }

  async function post() {
    console.log("post");
    ipfs.pubsub.publish(topic, createTopicPost(body));
    body = "";
  }

  async function openDialog() {
    console.log("openDialog");
  }

  function onTopicMessage(message) {
    let post = parsePubsubMessage(message);
    if (post != undefined) {
      posts = [post, ...posts];
    }
  }

  onMount(async () => {
    console.log("TopicFeed.onMount");
    await ipfs.pubsub.subscribe(topic, onTopicMessage);
  });

  onDestroy(async () => {
    console.log("TopicFeed.onDestroy");
    await ipfs.pubsub.unsubscribe(topic, onTopicMessage);
  });
</script>

<Form>
  <FormGroup>
    <FileDrop {handleFiles}>
      <TextArea
        bind:value={body}
        disabled={posting}
        labelText="/{topic}/"
        placeholder="What's happening?"
      />

      {#each files as file, i}
        <FileUploaderItem
          status="edit"
          name={file}
          on:delete={() => removeFile(i)}
        />
      {/each}
      <Button on:click={openDialog} disabled={posting}>Add files</Button>

      {#if !posting}
        <Button
          on:click={post}
          disabled={posting || (files.length < 1 && body.length < 1)}
        >
          Post
        </Button>
      {:else}
        <ProgressBar helperText="Publishing..." />
      {/if}
    </FileDrop>
  </FormGroup>
</Form>

{#each posts as post}
  <Tile>{post}</Tile>
{/each}
