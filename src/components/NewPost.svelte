<script lang="ts">
  import {
    Button,
    Form,
    FormGroup,
    TextArea,
    FileUploaderItem,
    Tile,
  } from "carbon-components-svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { readBinaryFile } from "@tauri-apps/api/fs";
  import { invoke } from "@tauri-apps/api/tauri";

  import type { AuxObj, PostRequest, PostResponse } from "../types.type";

  export let onPost: Function;

  let aux: AuxObj[] = [];
  let body: string = "";
  let files: string[] = [];
  let meta: string[] = [];

  async function post() {
    let postRequest: PostRequest = {
      aux: aux,
      body: body,
      files: files,
      meta: meta,
    };
    let postResponse: PostResponse = await invoke("post", {
      postRequest: postRequest,
    });
    if (postResponse) {
      onPost(postResponse);
      aux = [];
      body = "";
      files = [];
      meta = [];
    }
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
</script>

<Tile>
  <Form>
    <FormGroup>
      <TextArea
        labelText="New post"
        placeholder="What's happening?"
        bind:value={body}
      />

      <Button on:click={openDialog}>Add files</Button>
      {#each files as file}
        <FileUploaderItem status="complete" name={file} />
      {/each}
    </FormGroup>
    <Button on:click={post}>Post</Button>
  </Form>
</Tile>
