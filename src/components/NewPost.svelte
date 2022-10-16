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
  import type { Identity, Post, PostRequest, PostResponse } from "../types";
  import { addPost, getIdentityFromDB } from "../core";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { stripHtml } from "string-strip-html";

  export let onPost: Function;

  let identity: Identity;

  let body: string = "";
  let files: string[] = [];
  let meta: object = {};
  let posting = false;

  function arrayUnique(array) {
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
  }

  function removeFile(i) {
    files = files.slice(0, i).concat(files.slice(i + 1));
  }

  async function openDialog() {
    const res = await open({
      defaultPath: null,
      filters: [],
      multiple: true,
      directory: false,
    });
    console.log("openDialog.then");
    console.log(res);
    const bufs = [];
    if (Array.isArray(res)) {
      files = res;
      // res.forEach(async (path) => {
      //   console.log("path");
      //   console.log(path);
      //   const test = await readBinaryFile(path);
      //   const buf = Buffer.from(test);
      //   console.log(buf);
      // });
    }
  }

  async function post() {
    posting = true;
    if (body === "" && files.length === 1) {
      body = files[0].split(".").slice(0, -1).toString().split("/").pop();
    }
    let postRequest: PostRequest = {
      body: stripHtml(body).result,
      files: files,
      meta: meta,
      timestamp: new Date().getTime(),
    };
    let postResponse: PostResponse = await invoke("post", {
      request: postRequest,
    });
    console.log("post_response");
    console.log(postResponse);
    if (postResponse) {
      let post: Post = {
        ...postRequest,
        ...postResponse,
        display_name: identity.display_name,
        publisher: identity.publisher,
      };
      await addPost(post);
      onPost(post);
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
