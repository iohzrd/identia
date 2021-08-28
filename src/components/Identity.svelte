<script lang="ts">
  import {
    Button,
    ButtonSet,
    ButtonSkeleton,
    Column,
    Form,
    FormGroup,
    Grid,
    Row,
    TextInput,
    TextInputSkeleton,
    Tile,
  } from "carbon-components-svelte";
  // import { UserProfile20 } from "carbon-icons-svelte";

  import Post from "./Post.svelte";

  import { emit, listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";
  import { writable } from "svelte/store";

  type Identity = {
    aux: object[];
    av: string;
    dn: string;
    following: string[];
    meta: string[];
    posts: string[];
    publisher: string;
    ts: number;
  };

  export let identity: Identity;

  function onIdentityObject(obj) {
    console.log("onTestObj");
    console.log(obj);
    identity = obj;
  }

  function requestTestIdentity() {
    invoke("request_test_identity")
      .then(onIdentityObject)
      .catch(onIdentityObject);
  }

  function requestBlankIdentity() {
    invoke("get_identity", {
      publisher: "12D3KooWDED1CudLX9sdi1qBzy5tHS4Xi2Mpk45E5wrqteri1R8z",
    })
      .then(onIdentityObject)
      .catch(onIdentityObject);
  }

  function onPostObject(obj) {
    console.log("onPostObject");
    console.log(obj);
    // identity = obj;
  }

  function requestTestPost() {
    invoke("ipfs_get_post", {
      cid: "QmQW72f51MRFj9PaJnLPcUWkZXRMcQVctf1ExJXrU3wWRs",
    })
      .then(onPostObject)
      .catch(onPostObject);
  }
  function testManagedState() {
    invoke("test_managed_state")
      .then(onTestManagedState)
      .catch(onTestManagedState);
  }
  function onTestManagedState(obj) {
    console.log("onTestManagedState");
    console.log(obj);
  }
</script>

<Tile>
  <Form on:submit>
    <div>{window.location}</div>

    <button class="button" id="id" on:click={requestTestIdentity}>
      Request test identity (async)
    </button>

    <button class="button" id="id" on:click={requestBlankIdentity}>
      Request test identity (async)
    </button>

    <button class="button" id="id" on:click={requestTestPost}>
      Request test post (async)
    </button>

    <button class="button" id="id" on:click={testManagedState}>
      testManagedState
    </button>

    {#if identity}
      <FormGroup legendText="av">
        <!-- <UserProfile20>
          <title>Avitar</title>
        </UserProfile20> -->
        <!-- {identity["av"]} -->
      </FormGroup>

      <FormGroup>
        {#if identity.publisher}
          <TextInput
            disabled
            inline
            labelText="id"
            placeholder=""
            bind:value={identity["publisher"]}
          />
        {:else}
          <TextInputSkeleton />
        {/if}
      </FormGroup>

      <FormGroup>
        {#if identity.dn}
          <TextInput
            inline
            labelText="dn"
            placeholder=""
            bind:value={identity["dn"]}
          />
        {:else}
          <TextInputSkeleton />
        {/if}
      </FormGroup>

      <FormGroup legendText="aux">
        <Grid>
          {#each identity.aux as obj}
            <Row>
              {#if obj}
                <TextInput inline placeholder="" bind:value={obj["key"]} />
                <TextInput inline placeholder="" bind:value={obj["value"]} />
                <Button kind="primary">delete</Button>
              {:else}
                <TextInputSkeleton />
                <TextInputSkeleton />
                <ButtonSkeleton />
              {/if}
            </Row>
          {/each}
          <Button kind="primary">add</Button>
        </Grid>
      </FormGroup>

      <FormGroup legendText="following">
        {#if identity && identity.following}
          {#each identity.following as id}
            <div>
              {id}
            </div>
          {/each}
        {/if}
      </FormGroup>

      <FormGroup legendText="posts">
        {#if identity && identity.posts}
          {#each identity.posts as cid}
            <div>
              <Post {cid} includeFrom={false} />
            </div>
          {/each}
        {/if}
      </FormGroup>

      <FormGroup legendText="meta">
        {#if identity && identity.meta}
          {#each identity.meta as meta}
            {meta}
          {/each}
        {/if}
      </FormGroup>

      <FormGroup legendText="ts">
        {identity["ts"]}
      </FormGroup>
    {/if}
  </Form>

  <ButtonSet>
    <Button kind="secondary">Cancel</Button>
  </ButtonSet>
</Tile>
