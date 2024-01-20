<script lang="ts">
  import "../app.css";

  import { Button } from "flowbite-svelte";
  import { ButtonGroup } from "flowbite-svelte";
  import { Drawer } from "flowbite-svelte";
  import { Modal } from "flowbite-svelte";
  import { NavBrand } from "flowbite-svelte";
  import { NavHamburger } from "flowbite-svelte";
  import { NavLi } from "flowbite-svelte";
  import { NavUl } from "flowbite-svelte";
  import { Navbar } from "flowbite-svelte";
  import { Progressbar } from "flowbite-svelte";
  import { Sidebar } from "flowbite-svelte";
  import { SidebarDropdownItem } from "flowbite-svelte";
  import { SidebarDropdownWrapper } from "flowbite-svelte";
  import { SidebarGroup } from "flowbite-svelte";
  import { SidebarItem } from "flowbite-svelte";
  import { SidebarWrapper } from "flowbite-svelte";
  import { Spinner } from "flowbite-svelte";
  import { Textarea } from "flowbite-svelte";

  import { TrashBinSolid } from "flowbite-svelte-icons";
  import { UserAddOutline } from "flowbite-svelte-icons";
  import { UserSolid } from "flowbite-svelte-icons";
  import { sineIn } from "svelte/easing";

  import {
    followPublisher,
    getIdentity,
    ipfs,
    log,
    republishIdentity,
  } from "$lib/core";
  import type { IDResult } from "$lib/types";

  import { getTauriVersion, getVersion } from "@tauri-apps/api/app";
  import {
    deleteTopicFromDB,
    getTopicsFromDB,
    globalPubsubHandler,
    insertTopicIntoDB,
  } from "$lib/pubsub";
  import { goto } from "$app/navigation";
  import { multihash } from "is-ipfs";
  import { onMount, onDestroy } from "svelte";
  import { page } from "$app/stores";

  $: activeUrl = $page.url.pathname;
  let hidden = true;

  let ipfs_id: string;
  let ipfs_info: IDResult;
  let republishInterval: any;

  let app_version: string;
  let ipfs_version: string;
  let tauri_version: string;

  let follow_modal_open: boolean = false;
  let follow_waiting: boolean = false;
  let publisher_to_follow: string = "";
  $: publisher_invalid = !multihash(publisher_to_follow);

  let topic_modal_open = false;
  let topic_to_follow: string = "";
  let subs: string[] = [];

  function clearFollowModal() {
    follow_waiting = false;
    publisher_to_follow = "";
  }

  async function follow() {
    follow_waiting = true;
    await followPublisher(publisher_to_follow);
    clearFollowModal();
    follow_modal_open = false;
  }

  async function followTopic() {
    await insertTopicIntoDB(topic_to_follow);
    subs = await getTopicsFromDB();
    topic_modal_open = false;
    goto(`/topicfeed/${topic_to_follow}`);
  }

  async function unfollowTopic(topic: string) {
    await deleteTopicFromDB(topic);
    subs = await getTopicsFromDB();
  }

  onMount(async () => {
    app_version = await getVersion();
    tauri_version = await getTauriVersion();
    ipfs_info = await ipfs.id();
    ipfs_version = ipfs_info.agentVersion.split("/")[1];
    ipfs_id = ipfs_info.id.toString();
    await getIdentity(ipfs_id); // init identity if necessary...
    subs = await getTopicsFromDB();
    for await (const topic of [ipfs_id, ...subs]) {
      await ipfs.pubsub.subscribe(topic, globalPubsubHandler);
    }

    // periodically republish...
    // should refactor to republish based on previous
    await republishIdentity();
    republishInterval = setInterval(republishIdentity, 1000 * 60);
  });

  onDestroy(() => {
    clearInterval(republishInterval);
  });
</script>

<Navbar>
  <NavBrand class="dark:text-white">
    <button on:click={() => (hidden = false)}>
      <img alt="" class="me-3 h-6 sm:h-9" src="/favicon.png" />
    </button>
    <span class="self-center text-xl font-semibold dark:text-white">
      Identia
    </span>
  </NavBrand>

  <NavUl>
    <NavLi href="/identity/{ipfs_id}">Profile</NavLi>
    <NavLi on:click={() => (follow_modal_open = !follow_modal_open)}>
      Follow new identity
    </NavLi>
  </NavUl>
</Navbar>

<Drawer bind:hidden>
  <Sidebar {activeUrl}>
    <SidebarWrapper>
      <SidebarGroup>
        <SidebarItem href="/" label="Feed" />
        <SidebarItem href="/identity/{ipfs_id}" label="Identity" />
        <SidebarItem href="/webfeed" label="Web Feed" />
        <SidebarDropdownWrapper label="Topic Feeds" expanded>
          {#each subs as topic}
            <SidebarDropdownItem href="/topicfeed/{topic}" label="/{topic}/"
            ></SidebarDropdownItem>
            {#if $page.url.pathname === "/topicfeed/" + topic}
              <SidebarDropdownItem
                icon={TrashBinSolid}
                on:click={() => unfollowTopic(topic)}
              />
            {/if}
          {/each}
          {#if topic_modal_open}
            <SidebarDropdownItem label="/{topic_to_follow}/" />
          {/if}
          <SidebarDropdownItem
            label="new Topic"
            on:click={() => (topic_modal_open = !topic_modal_open)}
          />
        </SidebarDropdownWrapper>
      </SidebarGroup>

      <SidebarGroup border>
        <SidebarItem
          href="https://github.com/iohzrd/identia"
          label="identia: v{app_version}"
          target="_blank"
        />
        <SidebarItem
          href="https://github.com/ipfs/go-ipfs"
          label="ipfs: v{ipfs_version}"
          target="_blank"
        />
        <SidebarItem
          href="https://github.com/tauri-apps/tauri"
          label="tauri: v{tauri_version}"
          target="_blank"
        />
      </SidebarGroup>
    </SidebarWrapper>
  </Sidebar>
</Drawer>
<div class="grid grid-cols-1 justify-items-center">
  <slot />
</div>

<Modal bind:open={follow_modal_open}>
  <Textarea
    invalid={publisher_invalid}
    invalidText="Invalid IPNS id. Please try another."
    labelText="publisher to follow"
    placeholder="12D3KooW..."
    disabled={follow_waiting}
    bind:value={publisher_to_follow}
  />
  {#if follow_waiting}
    <Progressbar helperText="Please wait..." />
  {:else}
    <Button disabled={publisher_invalid} on:click={follow}>Follow</Button>
  {/if}
</Modal>

<Modal bind:open={topic_modal_open} on:close={() => (topic_to_follow = "")}>
  <Textarea
    invalid={false}
    invalidText=""
    labelText="topic to follow"
    placeholder="pol"
    disabled={false}
    bind:value={topic_to_follow}
  />
  <Button
    disabled={!topic_to_follow || subs.includes(topic_to_follow)}
    on:click={followTopic}>Follow Topic</Button
  >
</Modal>
