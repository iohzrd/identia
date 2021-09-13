<script lang="ts">
  import {
    Button,
    ButtonSet,
    Dropdown,
    ListItem,
    TextInput,
    Tile,
    UnorderedList,
  } from "carbon-components-svelte";
  import { onMount, onDestroy } from "svelte";

  export let meta = {};
  let new_meta_key = "";
  let new_meta_index = 0;
  let meta_types_custom = [
    { id: "1", text: "string", value: "" },
    { id: "2", text: "object", value: {} },
    { id: "3", text: "array", value: [] },
  ];

  let meta_types = [
    { id: "1", text: "string", value: "" },
    { id: "2", text: "object", value: {} },
    { id: "3", text: "custom", value: [] },
  ];

  function addMeta() {
    meta[new_meta_key] = meta_types_custom[new_meta_index].value;
  }

  function removeMeta(k) {
    let temp = meta;
    delete temp[k];
    meta = temp;
  }

  function isObject(o) {
    return o !== null && typeof o === "object" && Array.isArray(o) === false;
  }

  onMount(() => {});

  onDestroy(() => {});
</script>

{#each Object.keys(meta) as k}
  <ListItem>
    <ButtonSet>
      <div>
        {k}:
      </div>
      {#if isObject(meta[k])}
        <UnorderedList nested>
          <svelte:self meta={meta[k]} />
        </UnorderedList>
      {:else if Array.isArray(meta[k])}
        {meta[k]}
      {:else if typeof meta[k] === "string"}
        <TextInput size="sm" inline placeholder="" bind:value={meta[k]} />
      {:else}
        else: {meta[k]}
      {/if}
      <Button on:click={() => removeMeta(k)}>delete</Button>
    </ButtonSet>
  </ListItem>
{/each}
<ButtonSet>
  <Dropdown
    type="inline"
    size="sm"
    titleText="entry type"
    bind:selectedIndex={new_meta_index}
    items={meta_types}
  />
  <TextInput size="sm" placeholder="key" bind:value={new_meta_key} />
  <Dropdown
    type="inline"
    size="sm"
    titleText="entry type"
    bind:selectedIndex={new_meta_index}
    items={meta_types_custom}
  />
  <Button on:click={addMeta}>add entry</Button>
</ButtonSet>
