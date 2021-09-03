<script lang="ts">
  import {
    Button,
    ButtonSet,
    ButtonSkeleton,
    ClickableTile,
    Form,
    FormGroup,
    Grid,
    Row,
    TextInput,
    TextInputSkeleton,
    Tile,
  } from "carbon-components-svelte";
  // import { UserProfile20 } from "carbon-icons-svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  import Post from "./Post.svelte";
  import type { Identity, PostResponse } from "../types.type";

  export let ipfs_id: string;

  let identity: Identity;
  let posts: PostResponse[] = [];
  let posts_oldest_ts: number = Math.floor(new Date().getTime());
  let posts_limit: number = 10;
  $: posts_query = `SELECT cid,aux,body,files,meta,publisher,ts FROM posts WHERE publisher = '${ipfs_id}' AND ts < ${posts_oldest_ts} ORDER BY ts DESC LIMIT ${posts_limit}`;

  async function getPostsPage() {
    console.log(`getFeedPage: ${ipfs_id}`);
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
    identity = await invoke("get_identity", {
      publisher: ipfs_id,
    });
    await getPostsPage();
  });

  onDestroy(() => {});
</script>

<Tile>
  <Form on:submit>
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
