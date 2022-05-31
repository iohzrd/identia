<script lang="ts">
  import {
    Button,
    Content,
    Grid,
    Header,
    HeaderGlobalAction,
    HeaderUtilities,
    Loading,
    Modal,
    ProgressBar,
    SideNav,
    SideNavItems,
    SideNavLink,
    SkipToContent,
    TextInput,
  } from "carbon-components-svelte";
  import Add from "carbon-icons-svelte/lib/Add.svelte";
  import ExternalComponent from "./components/External.svelte";
  import FeedComponent from "./components/Feed.svelte";
  import IdentityComponent from "./components/Identity.svelte";
  import Router from "svelte-spa-router";
  import type { IDResult } from "ipfs-core-types/src/root";
  import { followPublisher, ipfs } from "./core";
  import { invoke } from "@tauri-apps/api";
  import { location } from "svelte-spa-router";
  import { multihash } from "is-ipfs";
  import { onMount, onDestroy } from "svelte";

  let isSideNavOpen = false;

  let ipfs_info: IDResult;
  let ipfs_id: string;

  let follow_modal_open = false;
  let follow_waiting = false;
  let publisher_to_follow: string = "";
  $: publisher_invalid = !multihash(publisher_to_follow);

  const views = [
    {
      label: "Feed",
      path: "/",
    },
    {
      label: "Identity",
      path: "/identity/",
    },
    {
      label: "External",
      path: "/external/",
    },
  ];

  const routes = {
    "/:publisher?": FeedComponent,
    "/identity/:publisher?": IdentityComponent,
    "/external/:publisher?": ExternalComponent,
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
    ipfs_info = await ipfs.id();
    ipfs_id = ipfs_info.id.toString();
    // let test = await invoke("fetch_external", {
    //   url: "https://lukesmith.xyz/rss.xml",
    // });
    // console.log("fetch_external test");
    // console.log(test);
  });

  onDestroy(() => {});
</script>

{#if ipfs_id}
  <Header
    bind:isSideNavOpen
    platformName="identia"
    persistentHamburgerMenu={true}
  >
    <svelte:fragment slot="skip-to-content">
      <SkipToContent />
    </svelte:fragment>

    <HeaderUtilities>
      <HeaderGlobalAction
        aria-label="Follow new identity"
        icon={Add}
        on:click={() => (follow_modal_open = true)}
      />
    </HeaderUtilities>
  </Header>

  <SideNav bind:isOpen={isSideNavOpen}>
    <SideNavItems>
      {#each views as view}
        <SideNavLink
          href="#{view.path}{ipfs_id}"
          text={view.label}
          isSelected={$location === view.path + ipfs_id}
        >
          {view.label}
        </SideNavLink>
      {/each}
    </SideNavItems>
  </SideNav>

  <Content>
    <Grid>
      <Router {routes} />
    </Grid>
  </Content>

  <Modal
    bind:open={follow_modal_open}
    modalHeading="Follow publisher"
    on:close={clearFollowModal}
    on:open
    passiveModal
    size="lg"
  >
    <TextInput
      invalid={publisher_invalid}
      invalidText="Invalid IPNS id. Please try another."
      labelText="publisher to follow"
      placeholder="12D3KooW..."
      disabled={follow_waiting}
      bind:value={publisher_to_follow}
    />
    {#if follow_waiting}
      <ProgressBar helperText="Please wait..." />
    {:else}
      <Button disabled={publisher_invalid} on:click={follow}>Follow</Button>
    {/if}
  </Modal>
{:else}
  <Loading />
{/if}
