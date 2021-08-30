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

  type PostRequest = {
    body: string;
    files: string[];
  };

  let body = "";
  let files = [];

  function post() {
    let postRequest: PostRequest = { body: body, files: files };
    invoke("post", { postRequest: postRequest })
      .then(onMessage)
      .catch(onMessage);
    console.log("send");
    console.log(body);
    console.log(files);
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

  function onDelete(event, arg2) {
    console.log("onDelete");
    console.log(event);
    console.log(arg2);
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
