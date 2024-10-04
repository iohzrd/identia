<script lang="ts">
  import {
    Button,
    FileUploaderItem,
    Form,
    FormGroup,
    ProgressBar,
    TextArea,
  } from "carbon-components-svelte";
  import FileDrop from "svelte-tauri-filedrop";
  import type { Identity, Post, PostRequest, PostResponse } from "$lib/types";
  import { addPost, getIdentityFromDB } from "$lib/core";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { stripHtml } from "string-strip-html";

  export let insertPostIntoFeed: Function;

  let identity: Identity;

  let body: string = "";
  let files: string[] = [];
  let meta: object = {};
  let posting = false;

  function getFilename(path: string) {
    return path.split("/").pop();
  }

  function arrayUnique(array: string[]) {
    let a = array.concat();
    for (let i = 0; i < a.length; ++i) {
      for (let j = i + 1; j < a.length; ++j) {
        if (a[i] === a[j]) a.splice(j--, 1);
      }
    }
    return a;
  }

  function handleFiles(paths: string[]) {
    paths = paths.map((path) => decodeURIComponent(path));
    files = arrayUnique([...files, ...paths]);
    if (body === "" && files.length > 0) {
      body = getFilename(files[0]);
    }
  }

  function removeFile(i: number) {
    files = files.slice(0, i).concat(files.slice(i + 1));
  }

  async function openDialog() {
    const res = await open({
      defaultPath: null,
      filters: [],
      multiple: true,
      directory: false,
    });
    if (Array.isArray(res)) {
      handleFiles(res);
    }
  }

  let stripOpts = {
    onlyStripTags: ["script", "style", "xml"],
    stripTogetherWithTheirContents: ["script", "style", "xml"],
  };
  async function post() {
    posting = true;
    let postRequest: PostRequest = {
      body: stripHtml(body, stripOpts).result,
      files: files,
      meta: meta,
      timestamp: new Date().getTime(),
    };
    let postResponse: PostResponse = await invoke("post", {
      request: postRequest,
    });
    if (postResponse) {
      let post: Post = {
        ...postRequest,
        ...postResponse,
        display_name: identity.display_name,
        publisher: identity.publisher,
      };
      await addPost(post);
      insertPostIntoFeed(post);
      body = "";
      files = [];
      meta = {};
    }
    posting = false;
  }

  onMount(async () => {
    identity = await getIdentityFromDB();
  });
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
