<script lang="ts">
  import {
    Button,
    ClickableTile,
    Form,
    FormGroup,
    TextArea,
    TextInput,
    Link,
  } from "carbon-components-svelte";
  import MediaModalComponent from "./MediaModal.svelte";
  import MetaComponent from "./Meta.svelte";
  import PostComponent from "./Post.svelte";
  import type { IDResult } from "ipfs-core-types/src/root";
  import type { Identity, Post } from "../types";
  import { onMount, onDestroy } from "svelte";
  import { getIdentity, ipfs, select, updateIdentity } from "../core";

  export let params = {};

  let ipfs_info: IDResult;
  let ipfs_id: string;

  let identity: Identity;
  let posts: Post[] = [];
  let posts_oldest_ts: number = new Date().getTime();
  let posts_limit: number = 5;
  $: publisher = params["publisher"];
  $: posts_query = `SELECT posts.cid, posts.body, posts.files, posts.meta, posts.publisher, posts.timestamp, identities.display_name FROM posts INNER JOIN identities ON identities.publisher = posts.publisher WHERE posts.publisher = '${publisher}' AND posts.timestamp < ${posts_oldest_ts} ORDER BY posts.timestamp DESC LIMIT ${posts_limit}`;

  let media_modal_idx = 0;
  let media_modal_media = [];
  let media_modal_open = false;

  async function getPostsPage() {
    console.log("getFeedPage: ", publisher);
    if (identity && identity.posts) {
      if (posts.length > 0) {
        posts_oldest_ts = posts[posts.length - 1].timestamp;
      }
      let page: Post[] = await select(posts_query);
      console.log("page:", page);
      if (page.length > 0) {
        posts = [...posts, ...page];
        posts_oldest_ts = posts[posts.length - 1].timestamp;
      }
    }
  }

  onMount(async () => {
    console.log("onMount");
    console.log(params);
    ipfs_info = await ipfs.id();
    ipfs_id = ipfs_info.id;
    if (params["publisher"]) {
      identity = await getIdentity(publisher);
    }
    await getPostsPage();
  });

  onDestroy(() => {
    ipfs_id = "";
    params = {};
  });
</script>

<MediaModalComponent
  bind:media_modal_idx
  bind:media_modal_media
  bind:media_modal_open
/>

<Form on:submit>
  {#if identity}
    <!-- <FormGroup legendText="avatar">
        <UserProfile20>
          <title>Avitar</title>
        </UserProfile20>
        {identity["avatar"]}
      </FormGroup> -->

    <FormGroup>
      {#if ipfs_id !== identity.publisher}
        <div>
          {identity["description"]}
        </div>
      {:else}
        <TextArea
          bind:value={identity["description"]}
          labelText="description"
          placeholder={ipfs_id === identity.publisher
            ? "Enter a description..."
            : ""}
        />
      {/if}
    </FormGroup>

    <FormGroup>
      <TextInput
        bind:value={identity["display_name"]}
        inline
        labelText="display name"
        placeholder=""
        readonly={ipfs_id !== identity.publisher}
      />
    </FormGroup>

    <FormGroup>
      <TextInput
        readonly
        inline
        labelText="publisher"
        placeholder=""
        bind:value={identity["publisher"]}
      />
    </FormGroup>

    <FormGroup legendText="following">
      {#if identity && identity.following}
        {#each identity.following as id}
          <div>
            <Link href="#/identity/{id}">{id}</Link>
          </div>
        {/each}
      {/if}
    </FormGroup>

    <FormGroup legendText="meta">
      {#if identity && identity.meta}
        <MetaComponent
          meta={identity.meta}
          readonly={ipfs_id !== identity.publisher}
        />
      {/if}
    </FormGroup>

    <FormGroup legendText="posts">
      {#if identity && identity.posts}
        {#each posts as post}
          <div>
            <PostComponent
              {ipfs_id}
              {post}
              bind:media_modal_idx
              bind:media_modal_media
              bind:media_modal_open
            />
          </div>
        {/each}
      {/if}
      <ClickableTile on:click={getPostsPage}>Load more posts</ClickableTile>
    </FormGroup>

    <FormGroup legendText="timestamp">
      {identity["timestamp"]}
    </FormGroup>

    <FormGroup legendText="last known cid">
      {identity["cid"]}
    </FormGroup>

    <Button
      on:click={() => {
        updateIdentity(identity);
      }}>Save</Button
    >
  {/if}
</Form>
