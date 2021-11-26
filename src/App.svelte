<script lang="ts">
  import {
    Button,
    ButtonSet,
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
  import Add20 from "carbon-icons-svelte/lib/Add20";
  import Feed from "./components/Feed.svelte";
  import Identity from "./components/Identity.svelte";
  import Router from "svelte-spa-router";
  import { invoke } from "@tauri-apps/api/tauri";
  import { location } from "svelte-spa-router";
  import { onMount, onDestroy } from "svelte";

  let follow_modal_open = false;
  let ipfs_id: string;
  let publisher_to_follow: string = "";
  let follow_success = false;
  let follow_waiting = false;

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
    console.log("followPublisher");
    follow_waiting = true;
    follow_success = await invoke("follow_publisher", {
      publisher: publisher_to_follow.trim(),
    });
    closeFollowModal();
  }

  function closeFollowModal() {
    follow_modal_open = false;
    follow_waiting = false;
    publisher_to_follow = "";
  }

  onMount(async () => {
    ipfs_id = await invoke("ipfs_id");
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
        icon={Add20}
        on:click={() => (follow_modal_open = !follow_modal_open)}
      />
    </HeaderUtilities>
  </Header>

  <Modal
    bind:open={follow_modal_open}
    modalHeading="Follow publisher"
    on:close
    on:open
    passiveModal
    preventCloseOnClickOutside
    size="lg"
  >
    <TextInput
      labelText="publisher to follow"
      placeholder="12D3KooW..."
      disabled={follow_waiting}
      bind:value={publisher_to_follow}
    />
    {#if follow_waiting}
      <ProgressBar helperText="Loading..." />
    {:else}
      <ButtonSet>
        <Button on:click={closeFollowModal} kind="secondary">Cancel</Button>
        <Button
          disabled={publisher_to_follow.length < 32}
          on:click={followPublisher}>Confirm</Button
        >
      </ButtonSet>
    {/if}
  </Modal>

  <Content>
    <Router {routes} />
  </Content>
{:else}
  <Loading />
{/if}
