<script lang="ts">
  import {
    Content,
    Header,
    HeaderGlobalAction,
    HeaderUtilities,
    SideNav,
    SideNavDivider,
    SideNavItems,
    SideNavLink,
    SkipToContent,
  } from "carbon-components-svelte";
  import SettingsAdjust20 from "carbon-icons-svelte/lib/SettingsAdjust20";
  import Add20 from "carbon-icons-svelte/lib/Add20";

  import External from "./components/External.svelte";
  import Identity from "./components/Identity.svelte";
  import Feed from "./components/Feed.svelte";
  import Settings from "./components/Settings.svelte";

  import { emit, listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  let isSideNavOpen = false;
  let ipfs_id: string;

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

  onMount(async () => {
    ipfs_id = await invoke("wait_for_ipfs_id_cmd");
    console.log("ipfs_id");
    console.log(ipfs_id);
  });

  onDestroy(() => {});
</script>

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
    <HeaderGlobalAction aria-label="Follow new identity" icon={Add20} />
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
  <svelte:component this={selected_view.component} {ipfs_id} />
</Content>
