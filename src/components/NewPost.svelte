<script lang="ts">
  import {
    Button,
    Form,
    FormGroup,
    TextArea,
    FileUploader,
    FileUploaderDropContainer,
    FileUploaderItem,
    Tile,
  } from "carbon-components-svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { readBinaryFile } from "@tauri-apps/api/fs";
  import { invoke } from "@tauri-apps/api/tauri";

  import type { PostRequest } from "../types.type";

  let aux = [];
  let body = "";
  let files = [];
  let meta = [];

  function post() {
    let postRequest: PostRequest = {
      aux: aux,
      body: body,
      files: files,
      meta: meta,
    };
    invoke("post", { postRequest: postRequest })
      .then(onMessage)
      .catch(onMessage);
  }
  function onMessage(msg) {
    console.log("onMessage");
    console.log(msg);
  }

  function openDialog() {
    open({
      defaultPath: null,
      filters: [],
      multiple: true,
      directory: false,
    })
      .then(function (res) {
        onMessage("then");
        onMessage(res);
        if (Array.isArray(res)) {
          files = res;
        }
      })
      .catch(onMessage);
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
