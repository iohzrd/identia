<script lang="ts">
  import {
    Button,
    ButtonSet,
    ClickableTile,
    Form,
    FormGroup,
    TextArea,
    TextInput,
    Tile,
    UnorderedList,
  } from "carbon-components-svelte";
  // import { UserProfile20 } from "carbon-icons-svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  import Post from "./Post.svelte";
  import Meta from "./Meta.svelte";
  import type { Identity, PostResponse } from "../types.type";

  export let params = {};

  let identity: Identity;
  let posts: PostResponse[] = [];
  let posts_oldest_ts: number = Math.floor(new Date().getTime());
  let posts_limit: number = 5;
  $: publisher = params["publisher"];
  $: posts_query = `SELECT cid,body,files,meta,publisher,timestamp FROM posts WHERE publisher = '${publisher}' AND timestamp < ${posts_oldest_ts} ORDER BY timestamp DESC LIMIT ${posts_limit}`;

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

  onMount(async () => {
    console.log("onMount");
    console.log(params);
    if (params["publisher"]) {
      identity = await invoke("get_identity", {
        publisher: params["publisher"],
      });
    }
    await getPostsPage();

    if (identity && identity.meta) {
      if (Array.isArray(identity.meta)) {
        identity.meta = {};
      }
    }
  });

  onDestroy(() => {});
</script>

<Form on:submit>
  {#if identity}
    <!-- <FormGroup legendText="avatar">
        <UserProfile20>
          <title>Avitar</title>
        </UserProfile20>
        {identity["avatar"]}
      </FormGroup> -->

    <FormGroup>
      <TextArea
        labelText="description"
        placeholder="Enter a description..."
        bind:value={identity["description"]}
      />
    </FormGroup>

    <FormGroup>
      <TextInput
        inline
        labelText="display name"
        placeholder=""
        bind:value={identity["display_name"]}
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
            {id}
          </div>
        {/each}
      {/if}
    </FormGroup>

    <FormGroup legendText="meta">
      {#if identity && identity.meta}
        <UnorderedList>
          <Meta meta={identity.meta} />
        </UnorderedList>
      {/if}
    </FormGroup>

    <FormGroup legendText="posts">
      {#if identity && identity.posts}
        {#each posts as postResponse}
          <div>
            <Post cid={null} {postResponse} includeFrom={false} />
          </div>
        {/each}
      {/if}
      <ClickableTile on:click={getPostsPage}>Load more posts</ClickableTile>
    </FormGroup>

    <FormGroup legendText="timestamp">
      {identity["timestamp"]}
    </FormGroup>
  {/if}
</Form>
