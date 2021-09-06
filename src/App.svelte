<script lang="ts">
  import {
    Button,
    Content,
    Header,
    HeaderGlobalAction,
    HeaderUtilities,
    Loading,
    Modal,
    SideNav,
    SideNavDivider,
    SideNavItems,
    SideNavLink,
    SkipToContent,
    TextInput,
  } from "carbon-components-svelte";
  import SettingsAdjust20 from "carbon-icons-svelte/lib/SettingsAdjust20";
  import Add20 from "carbon-icons-svelte/lib/Add20";

  import External from "./components/External.svelte";
  import Identity from "./components/Identity.svelte";
  import Feed from "./components/Feed.svelte";
  import Settings from "./components/Settings.svelte";

  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  let add_diaglog_open = false;
  let isSideNavOpen = false;
  let ipfs_id: string;
  let publisher_to_follow: string;

  const views = [
    {
      label: "Feed",
      component: Feed,
    },
    {
      label: "Identity",
      component: Identity,
    },
    {
      label: "Settings",
      component: Settings,
    },
    {
      label: "External",
      component: External,
    },
  ];
  let selected_view = views[0];

  function select(view) {
    selected_view = view;
  }

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
  <Header
    persistentHamburgerMenu={true}
    company="Identia: "
    platformName={ipfs_id}
    bind:isSideNavOpen
  >
    <div slot="skip-to-content">
      <SkipToContent />
    </div>
    <HeaderUtilities>
      <HeaderGlobalAction aria-label="Settings" icon={SettingsAdjust20} />
      <HeaderGlobalAction
        aria-label="Follow new identity"
        icon={Add20}
        on:click={() => (add_diaglog_open = !add_diaglog_open)}
      />
    </HeaderUtilities>
  </Header>

  <SideNav bind:isOpen={isSideNavOpen}>
    <SideNavItems>
      {#each views as view}
        <SideNavLink
          text={view.label}
          class="nv noselect {selected_view === view ? 'nv_selected' : ''}"
          on:click={() => select(view)}
        >
          {view.label}
        </SideNavLink>
      {/each}
      <SideNavDivider />
    </SideNavItems>
  </SideNav>

  <Content>
    <Modal
      size="lg"
      bind:open={add_diaglog_open}
      modalHeading="Create database"
      primaryButtonText="Confirm"
      secondaryButtonText="Cancel"
      on:click:button--secondary
      on:open
      on:close={() => (add_diaglog_open = false)}
      on:submit={followPublisher}
    >
      <TextInput
        labelText="publisher to follow"
        placeholder="Q..."
        bind:value={publisher_to_follow}
      />
    </Modal>

    <svelte:component this={selected_view.component} {ipfs_id} />
  </Content>
{:else}
  <Loading />
{/if}
