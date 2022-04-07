<script lang="ts">
  import {
    Button,
    Content,
    Header,
    HeaderGlobalAction,
    HeaderNav,
    HeaderNavItem,
    HeaderUtilities,
    Loading,
    Modal,
    ProgressBar,
    SkipToContent,
    TextInput,
  } from "carbon-components-svelte";
  // import Database from "tauri-plugin-sql-api";
  import Add from "carbon-icons-svelte/lib/Add.svelte";
  import FeedComponent from "./components/Feed.svelte";
  import IdentityComponent from "./components/Identity.svelte";
  import Router from "svelte-spa-router";
  import type { IDResult } from "ipfs-core-types/src/root";
  import type { IPFSHTTPClient } from "ipfs-http-client";
  import type { Identity } from "./types.type";
  import { create } from "ipfs-http-client";
  import { followPublisher, getIdentity } from "./Core.svelte";
  import { location } from "svelte-spa-router";
  import { multihash } from "is-ipfs";
  import { onMount, onDestroy } from "svelte";

  let identity: Identity;
  let ipfs: IPFSHTTPClient;
  let ipfs_info: IDResult;
  let ipfs_id: string;

  let follow_modal_open = false;
  let follow_waiting = false;
  let publisher_to_follow: string = "";
  $: publisher_to_follow_invalid = !multihash(publisher_to_follow);

  const views = [
    {
      label: "Feed",
      path: "/",
    },
    {
      label: "Identity",
      path: "/identity/",
    },
  ];

  const routes = {
    "/:publisher?": FeedComponent,
    "/identity/:publisher?": IdentityComponent,
    // "*": NotFound,
  };

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

  onMount(async () => {
    ipfs = await create({ url: "/ip4/127.0.0.1/tcp/5001" });
    ipfs_info = await ipfs.id();
    ipfs_id = ipfs_info.id;
    // const db = await Database.load(`sqlite:${ipfs_id}.db`);
    // 12D3KooWHxU85q4JWsDXq4ZHjBCdjHHGL9wnMtqBMMgArkn6xcyz
    identity = await getIdentity(ipfs_id);
  });

  onDestroy(() => {});
</script>

{#if ipfs_id}
  <Header company="identia">
    <div slot="skip-to-content">
      <SkipToContent />
    </div>

    <HeaderNav>
      {#each views as view}
        <HeaderNavItem
          href="#{view.path}{ipfs_id}"
          text={view.label}
          isSelected={$location === view.path + ipfs_id}
        >
          {view.label}
        </HeaderNavItem>
      {/each}
    </HeaderNav>

    <HeaderUtilities>
      <HeaderGlobalAction
        aria-label="Follow new identity"
        icon={Add}
        on:click={() => (follow_modal_open = true)}
      />
    </HeaderUtilities>
  </Header>

  <Modal
    bind:open={follow_modal_open}
    modalHeading="Follow publisher"
    on:close={clearFollowModal}
    on:open
    passiveModal
    size="lg"
  >
    <TextInput
      invalid={publisher_to_follow_invalid}
      invalidText="Invalid IPNS id. Please try another."
      labelText="publisher to follow"
      placeholder="12D3KooW..."
      disabled={follow_waiting}
      bind:value={publisher_to_follow}
    />
    {#if follow_waiting}
      <ProgressBar helperText="Please wait..." />
    {:else}
      <Button disabled={publisher_to_follow_invalid} on:click={follow}
        >Follow</Button
      >
    {/if}
  </Modal>

  <Content>
    <Router {routes} />
  </Content>
{:else}
  <Loading />
{/if}
