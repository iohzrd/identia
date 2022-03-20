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
  // import { UserProfile20 } from "carbon-icons-svelte";
  import MediaModal from "./MediaModal.svelte";
  import Meta from "./Meta.svelte";
  import Post from "./Post.svelte";
  import type { Identity, IdentityResponse, PostResponse } from "../types.type";
  import { create } from "ipfs-http-client/index";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  export let params = {};

  let ipfs;
  let ipfs_info;
  let ipfs_id = "";
  let identity_res: IdentityResponse;
  let identity: Identity;
  let posts: PostResponse[] = [];
  let posts_oldest_ts: number = Math.floor(new Date().getTime());
  let posts_limit: number = 5;
  $: publisher = params["publisher"];
  $: posts_query = `SELECT posts.cid, posts.body, posts.files, posts.meta, posts.publisher, posts.timestamp, identities.display_name FROM posts INNER JOIN identities ON identities.publisher = posts.publisher WHERE posts.publisher = '${publisher}' AND posts.timestamp < ${posts_oldest_ts} ORDER BY posts.timestamp DESC LIMIT ${posts_limit}`;

  let media_modal_idx = 0;
  let media_modal_media = [];
  let media_modal_open = false;

  async function getPostsPage() {
    console.log(`getFeedPage: ${publisher}`);
    if (identity && identity.posts) {
      if (posts.length > 0) {
        posts_oldest_ts = posts[posts.length - 1].post.timestamp;
      }
      let page: PostResponse[] = await invoke("query_posts", {
        query: posts_query,
      });
      if (page.length > 0) {
        posts = [...posts, ...page];
        posts_oldest_ts = posts[posts.length - 1].post.timestamp;
      }
    }
  }

  async function updateIdentityAux() {
    console.log(`updateMeta`);
    let identity_res: IdentityResponse[] = await invoke("update_identity_aux", {
      desc: identity.description,
      dn: identity.display_name,
      meta: identity.meta,
    });
    console.log(identity_res);
  }

  onMount(async () => {
    console.log("onMount");
    console.log(params);
    ipfs = await create("/ip4/127.0.0.1/tcp/5001");
    ipfs_info = await ipfs.id();
    ipfs_id = ipfs_info.id;
    if (params["publisher"]) {
      identity_res = await invoke("get_identity", {
        publisher: params["publisher"],
      });
      identity = identity_res.identity;
    }
    await getPostsPage();

    if (identity && identity.meta) {
      if (Array.isArray(identity.meta)) {
        identity.meta = {};
      }
    }
  });

  onDestroy(() => {
    ipfs_id = "";
    params = {};
  });
</script>

<MediaModal bind:media_modal_idx bind:media_modal_media bind:media_modal_open />

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
        <Meta meta={identity.meta} readonly={ipfs_id !== identity.publisher} />
      {/if}
    </FormGroup>

    <FormGroup legendText="posts">
      {#if identity && identity.posts}
        {#each posts as post_response}
          <div>
            <Post
              {ipfs_id}
              cid={null}
              {post_response}
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
      {identity_res["cid"]}
    </FormGroup>

    <Button
      on:click={() => {
        updateIdentityAux();
      }}>Save</Button
    >
  {/if}
</Form>
