<script lang="ts">
  import type { PageData } from "./$types";

  import { Avatar } from "flowbite-svelte";
  import { Button } from "flowbite-svelte";
  import { Card } from "flowbite-svelte";
  import { Textarea } from "flowbite-svelte";
  import { Listgroup } from "flowbite-svelte";
  import { ListgroupItem } from "flowbite-svelte";

  import MediaModalComponent from "$lib/MediaModal.svelte";
  import MetaComponent from "$lib/Meta.svelte";
  import PostComponent from "$lib/Post.svelte";
  import type { IDResult } from "$lib/types";
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
  bind:media={media_modal_media}
  bind:open={media_modal_open}
/>

<div on:submit>
  {#if identity}
    <!-- <div legendText="avatar">
        <UserProfile20>
          <title>Avitar</title>
        </UserProfile20>
        {identity["avatar"]}
      </div> -->

    <div>
      <Textarea
        bind:value={identity["description"]}
        labelText="description"
        readonly={ipfs_id !== identity.publisher}
        placeholder={ipfs_id === identity.publisher
          ? "Enter a description..."
          : ""}
      />
    </div>

    <div>
      <Textarea
        bind:value={identity["display_name"]}
        inline
        labelText="display name"
        placeholder=""
        readonly={ipfs_id !== identity.publisher}
      />
    </div>

    <div>
      <Textarea
        readonly
        inline
        labelText="publisher"
        placeholder=""
        bind:value={identity.publisher}
      />
    </div>

    <Listgroup>
      <h3 class="p-1 text-center text-xl font-medium dark:text-white">
        following
      </h3>
      {#each following as identity}
        <ListgroupItem class="text-base font-semibold gap-2">
          <Avatar src="" size="xs" />
          <a href="/identity/{identity.publisher}">
            {#if identity.display_name}
              {identity.display_name} ({identity.publisher})
            {:else}
              {identity.publisher}
            {/if}
          </a>
        </ListgroupItem>
      {/each}
    </Listgroup>

    <Listgroup>
      {#if identity && identity.meta}
        <h3 class="p-1 text-center text-xl font-medium dark:text-white">
          meta
        </h3>

        <MetaComponent
          meta={identity.meta}
          readonly={ipfs_id !== identity.publisher}
        />
      {/if}
    </Listgroup>

    <Listgroup>
      <h3 class="p-1 text-center text-xl font-medium dark:text-white">posts</h3>
      {#if identity && identity.posts}
        {#each posts as post (post.cid)}
          <ListgroupItem>
            <PostComponent {ipfs_id} {post} />
          </ListgroupItem>
        {/each}
      {/if}
      <Card on:click={getPostsPage}>Load more posts</Card>
    </Listgroup>

    <Listgroup>
      <ListgroupItem legendText="Last published">
        {timeago} ({datetime}) - ({identity.timestamp})
      </ListgroupItem>

      <ListgroupItem legendText="last known cid">
        {identity["cid"]}
      </ListgroupItem>
    </Listgroup>

    <Button
      disabled={ipfs_id !== identity.publisher}
      on:click={() => {
        updateIdentity(identity);
      }}>Save</Button
    >
  {/if}
</div>
