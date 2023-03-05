<script lang="ts">
  import type { PageData } from "./$types";
  import {
    Button,
    ClickableTile,
    Form,
    FormGroup,
    TextArea,
    TextInput,
    Link,
  } from "carbon-components-svelte";
  import MediaModalComponent from "$lib/MediaModal.svelte";
  import MetaComponent from "$lib/Meta.svelte";
  import PostComponent from "$lib/Post.svelte";
  import type { IDResult } from "ipfs-core-types/src/root";
  import type { Identity, Post } from "$lib/types";
  import { format as formatTime } from "timeago.js";
  import { getIdentity, ipfs, log, updateIdentity } from "$lib/core";
  import { onMount, onDestroy } from "svelte";
  import { select } from "$lib/db";

  export let data: PageData;

  let ipfs_info: IDResult;
  let ipfs_id: string;

  let identity: Identity;
  $: timeago = formatTime(identity ? identity["timestamp"] : 0);
  $: datetime = new Date(identity ? identity["timestamp"] : 0).toLocaleString();

  let posts: Post[] = [];
  let following: Identity[] = [];
  let posts_oldest_ts: number = new Date().getTime();
  let posts_limit: number = 5;
  $: posts_query = `SELECT posts.cid, posts.body, posts.files, posts.meta, posts.publisher, posts.timestamp, identities.display_name FROM posts INNER JOIN identities ON identities.publisher = posts.publisher WHERE posts.publisher = '${data.publisher}' AND posts.timestamp < ${posts_oldest_ts} ORDER BY posts.timestamp DESC LIMIT ${posts_limit}`;

  let media_modal_idx = 0;
  let media_modal_media = [];
  let media_modal_open = false;

  async function getPostsPage() {
    if (identity && identity.posts) {
      if (posts.length > 0) {
        posts_oldest_ts = posts[posts.length - 1].timestamp;
      }
      let page: Post[] = await select(posts_query);
      if (page.length > 0) {
        posts = [...posts, ...page];
        posts_oldest_ts = posts[posts.length - 1].timestamp;
      }
    }
  }

  onMount(async () => {
    ipfs_info = await ipfs.id();
    ipfs_id = ipfs_info.id.toString();
    if (data.publisher) {
      identity = await getIdentity(data.publisher);
      following = await Promise.all(
        identity.following.map(async (publisher) => {
          return await getIdentity(publisher);
        })
      );
    }
    await getPostsPage();
  });

  onDestroy(() => {
    ipfs_id = "";
  });
</script>

<MediaModalComponent
  bind:start={media_modal_idx}
  bind:media={media_modal_media}
  bind:open={media_modal_open}
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
      <TextArea
        bind:value={identity["description"]}
        labelText="description"
        readonly={ipfs_id !== identity.publisher}
        placeholder={ipfs_id === identity.publisher
          ? "Enter a description..."
          : ""}
      />
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
        bind:value={identity.publisher}
      />
    </FormGroup>

    <FormGroup legendText="following">
      {#if following}
        {#each following as identity}
          <div>
            <Link href="/identity/{identity.publisher}">
              {#if identity.display_name}
                {identity.display_name} ({identity.publisher})
              {:else}
                {identity.publisher}
              {/if}
            </Link>
          </div>
        {/each}
      {:else if identity && identity.following}
        {#each identity.following as id}
          <div>
            <Link href="/identity/{id}">
              {id}
            </Link>
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
        {#each posts as post (post.cid)}
          <PostComponent {ipfs_id} {post} />
        {/each}
      {/if}
      <ClickableTile on:click={getPostsPage}>Load more posts</ClickableTile>
    </FormGroup>

    <FormGroup legendText="Last published">
      {timeago} ({datetime}) - ({identity.timestamp})
    </FormGroup>

    <FormGroup legendText="last known cid">
      {identity["cid"]}
    </FormGroup>

    <Button
      disabled={ipfs_id !== identity.publisher}
      on:click={() => {
        updateIdentity(identity);
      }}>Save</Button
    >
  {/if}
</Form>
