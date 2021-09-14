<script lang="ts">
  import {
    Content,
    Header,
    HeaderGlobalAction,
    HeaderUtilities,
    Loading,
    Modal,
    SkipToContent,
    HeaderNav,
    HeaderNavItem,
    TextInput,
  } from "carbon-components-svelte";
  import Add20 from "carbon-icons-svelte/lib/Add20";

  import Identity from "./components/Identity.svelte";
  import Feed from "./components/Feed.svelte";

  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";
  import Router from "svelte-spa-router";
  import { location } from "svelte-spa-router";

  let add_diaglog_open = false;
  let ipfs_id: string;
  let publisher_to_follow: string;

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
    "/:publisher?": Feed,
    "/identity/:publisher?": Identity,
    // "*": NotFound,
  };

  async function followPublisher() {
    let success = await invoke("follow_publisher", {
      publisher: publisher_to_follow,
    });
    publisher_to_follow = "";
    console.log(`followPublisher: ${success}`);
  }

  onMount(async () => {
    ipfs_id = await invoke("ipfs_id");
    console.log("ipfs_id");
    console.log(ipfs_id);
  });

  onDestroy(() => {});
</script>

{#if ipfs_id}
  <Header company="Identia: " platformName={ipfs_id}>
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
        icon={Add20}
        on:click={() => (add_diaglog_open = !add_diaglog_open)}
      />
    </HeaderUtilities>
  </Header>

  <Modal
    size="lg"
    bind:open={add_diaglog_open}
    modalHeading="Follow publisher"
    primaryButtonText="Confirm"
    secondaryButtonText="Cancel"
    on:click:button--secondary
    on:open
    on:close={() => (add_diaglog_open = false)}
    on:submit={followPublisher}
  >
    <TextInput
      labelText="publisher to follow"
      placeholder="12D3KooW..."
      bind:value={publisher_to_follow}
    />
  </Modal>

  <Content>
    <Router {routes} />
  </Content>
{:else}
  <Loading />
{/if}
