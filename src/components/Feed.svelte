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

  import NewPost from "./NewPost.svelte";
  import Post from "./Post.svelte";

  let feed = [];
</script>

<Tile>
  <NewPost />

  {#if feed}
    {#each feed as cid}
      <div>
        <Post {cid} includeFrom={true} />
      </div>
    {/each}
  {/if}
</Tile>
