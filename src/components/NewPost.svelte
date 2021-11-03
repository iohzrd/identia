<script lang="ts">
  import type { PostRequest, PostResponse } from "../types.type";
  import {
    Button,
    FileUploaderItem,
    Form,
    FormGroup,
    ProgressBar,
    TextArea,
  } from "carbon-components-svelte";
  import { onMount, onDestroy } from "svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";
  import { stripHtml } from "string-strip-html";

  export let onPost: Function;

  let body: string = "";
  let files: string[] = [];
  let meta: object = {};
  let awaiting_response = false;

  async function post() {
    awaiting_response = true;
    let postRequest: PostRequest = {
      body: stripHtml(body).result,
      files: files,
      meta: meta,
    };
    let postResponse: PostResponse = await invoke("post", {
      postRequest: postRequest,
    });
    if (postResponse) {
      onPost(postResponse);
      body = "";
      files = [];
      meta = {};
    }
    awaiting_response = false;
  }

  function openDialog() {
    open({
      defaultPath: null,
      filters: [],
      multiple: true,
      directory: false,
    }).then(function (res) {
      console.log("openDialog.then");
      console.log(res);
      if (Array.isArray(res)) {
        files = res;
      }
    });
  }

  onMount(async () => {});

  onDestroy(() => {});
</script>

<Form>
  <FormGroup>
    <TextArea
      labelText="New post"
      placeholder="What's happening?"
      bind:value={body}
      bind:disabled={awaiting_response}
    />
    {#each files as file}
      <FileUploaderItem status="complete" name={file} />
    {/each}
    <Button on:click={openDialog} bind:disabled={awaiting_response}
      >Add files</Button
    >

    {#if !awaiting_response}
      <Button on:click={post} bind:disabled={awaiting_response}>Post</Button>
    {:else}
      <ProgressBar helperText="Publishing..." />
    {/if}
  </FormGroup>
</Form>
