<script lang="ts">
  import {
    Button,
    ButtonSet,
    ClickableTile,
    Form,
    FormGroup,
    TextInput,
    Tile,
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
  $: posts_query = `SELECT cid,body,files,meta,publisher,ts FROM posts WHERE publisher = '${publisher}' AND ts < ${posts_oldest_ts} ORDER BY ts DESC LIMIT ${posts_limit}`;

  async function getPostsPage() {
    console.log(`getFeedPage: ${publisher}`);
    if (identity && identity.posts) {
      if (posts.length > 0) {
        posts_oldest_ts = posts[posts.length - 1].post.ts;
      }
      let page: PostResponse[] = await invoke("query_posts", {
        query: posts_query,
      });
      if (page.length > 0) {
        posts = [...posts, ...page];
        posts_oldest_ts = posts[posts.length - 1].post.ts;
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

<Tile>
  <Form on:submit>
    {#if identity}
      <!-- <FormGroup legendText="av">
        <UserProfile20>
          <title>Avitar</title>
        </UserProfile20>
        {identity["av"]}
      </FormGroup> -->

      <FormGroup>
        <TextInput
          inline
          labelText="dn"
          placeholder=""
          bind:value={identity["dn"]}
        />
      </FormGroup>

      <FormGroup>
        <TextInput
          disabled
          inline
          labelText="id"
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

      {#if identity && identity.meta}
        <Meta meta={identity.meta} />
        <Button on:click={() => console.log(identity.meta)}>print</Button>
      {/if}

      <FormGroup legendText="posts">
        {#if identity && identity.posts}
          {#each posts as postResponse}
            <div>
              <Post cid={null} {postResponse} includeFrom={false} />
            </div>
          {/each}
        {/if}
        <ClickableTile light on:click={getPostsPage}
          >Load more posts</ClickableTile
        >
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
