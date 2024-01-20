<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { ButtonGroup } from "flowbite-svelte";
  import { Dropdown } from "flowbite-svelte";
  import { Listgroup } from "flowbite-svelte";
  import { ListgroupItem } from "flowbite-svelte";
  import { Textarea } from "flowbite-svelte";

  import { onMount, onDestroy } from "svelte";

  export let readonly: boolean;
  export let meta: { [k: string]: any } = {};
  export let depth: number = 0;
  let new_meta_key: string = "";
  let new_meta_index: number = 0;
  let meta_types = [
    { id: 0, text: "string", value: "" },
    { id: 1, text: "object", value: {} },
    { id: 2, text: "custom", value: [] },
  ];

  function addMeta() {
    meta[new_meta_key] = meta_types[new_meta_index].value;
  }

  function removeMeta(k: string) {
    let temp = meta;
    delete temp[k];
    meta = temp;
  }

  function isObject(o: any) {
    return o !== null && typeof o === "object" && Array.isArray(o) === false;
  }

  onMount(() => {});

  onDestroy(() => {});
</script>

<Listgroup>
  {#each Object.keys(meta) as k}
    <ListgroupItem>
      <ButtonGroup>
        <div>
          {k}:
        </div>

        {#if isObject(meta[k])}
          <svelte:self meta={meta[k]} {readonly} depth={depth + 1} />
        {:else if Array.isArray(meta[k])}
          {meta[k]}
        {:else if typeof meta[k] === "string"}
          <Textarea inline placeholder="" bind:value={meta[k]} {readonly} />
        {:else}
          else: {meta[k]}
        {/if}

        {#if !readonly}
          <Button on:click={() => removeMeta(k)}>delete</Button>
        {/if}
      </ButtonGroup>
    </ListgroupItem>
  {/each}
</Listgroup>

<br />
{#if !readonly}
  <ButtonGroup>
    <Dropdown type="inline" />
    <Textarea class="input" placeholder="key" bind:value={new_meta_key} />
    <Button class="button" on:click={addMeta}>add entry</Button>
  </ButtonGroup>
{/if}
