<script lang="ts">
  import {
    Tile,
    TextInput,
    Form,
    FormGroup,
    Checkbox,
    RadioButtonGroup,
    RadioButton,
    Select,
    SelectItem,
    Button,
  } from "carbon-components-svelte";
  // import { UserProfile20 } from "carbon-icons-svelte";

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

  function requestTestObj() {
    invoke("request_test_identity")
      .then(onIdentityObject)
      .catch(onIdentityObject);
  }
</script>

<Tile light>
  <Form on:submit>
    <div>{window.location}</div>

    <button class="button" id="id" on:click={requestTestObj}>
      Request obj (async)
    </button>

    {#if identity}
      <FormGroup legendText="av">
        <!-- <UserProfile20>
          <title>Avitar</title>
        </UserProfile20> -->
        <!-- {identity["av"]} -->
      </FormGroup>

      <FormGroup>
        <TextInput
          inline
          labelText="dn"
          placeholder=""
          bind:value={identity["dn"]}
        />
      </FormGroup>

      <FormGroup legendText="aux">
        {#each identity.aux as obj}
          <div>
            <TextInput
              inline
              labelText="key"
              placeholder=""
              bind:value={obj["key"]}
            />
            <TextInput
              inline
              labelText="value"
              placeholder=""
              bind:value={obj["value"]}
            />
          </div>
        {/each}
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
          {#each identity.posts as post}
            <div>
              {post}
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

      <FormGroup legendText="publisher">
        {identity["publisher"]}
      </FormGroup>

      <FormGroup legendText="ts">
        {identity["ts"]}
      </FormGroup>
    {/if}
  </Form>
</Tile>
