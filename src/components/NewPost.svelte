<script lang="ts">
  import FileDrop from "svelte-tauri-filedrop";
  import type { PostRequest, PostResponse } from "../types.type";
  import {
    Button,
    FileUploaderItem,
    Form,
    FormGroup,
    ProgressBar,
    TextArea,
  } from "carbon-components-svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { stripHtml } from "string-strip-html";

  export let onPost: Function;

  let body: string = "";
  let files: string[] = [];
  let meta: object = {};
  let posting = false;

  async function post() {
    posting = true;
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
    posting = false;
  }

  function arrayUnique(array) {
    let a = array.concat();
    for (let i = 0; i < a.length; ++i) {
      for (let j = i + 1; j < a.length; ++j) {
        if (a[i] === a[j]) a.splice(j--, 1);
      }
    }

    return a;
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

  function handleFiles(paths: string[]) {
    paths = paths.map((path) => decodeURIComponent(path));
    files = arrayUnique([...files, ...paths]);
  }

  onMount(async () => {});
  onDestroy(() => {});
</script>

<Form>
  <FormGroup>
    <FileDrop {handleFiles}>
      <TextArea
        bind:value={body}
        disabled={posting}
        labelText="New post"
        placeholder="What's happening?"
      />

      {#each files as file}
        <FileUploaderItem status="complete" name={file} />
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
