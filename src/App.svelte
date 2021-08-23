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
  import { writable } from "svelte/store";

  // export let onIpfsId;
  let isSideNavOpen = false;
  let ipfs_id_unlisten;
  let ipfs_id = "";
  let responses = writable([]);

  const views = [
    {
      label: "Identity",
      component: Identity,
    },
    {
      label: "Feed",
      component: Feed,
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

  function log() {
    invoke("log_operation", {
      event: "tauri-click",
      payload: "this payload is optional because we used Option in Rust",
    });
  }

  function performRequest() {
    invoke("perform_request", {
      endpoint: "dummy endpoint arg",
      body: {
        id: 5,
        name: "test",
      },
    })
      .then(onIpfsId)
      .catch(onIpfsId);
  }

  function requestTestObj() {
    invoke("request_test_identity").then(onTestObj).catch(onTestObj);
  }

  function onTestObj(obj) {
    console.log("onTestObj");
    console.log(obj);
  }

  function ipfsID() {
    invoke("ipfs_id").then(onIpfsId).catch(onIpfsId);
  }

  function emitEvent() {
    emit("ipfs-id", "this is the payload string");
  }

  function onIpfsId(value) {
    console.log("onIpfsId");
    if (
      value &&
      value.payload &&
      value.payload.data &&
      typeof value.payload.data === "string"
    ) {
      ipfs_id = value.payload.data;
    }
  }

  function onMessage(value) {
    responses.update((r) => [
      `[${new Date().toLocaleTimeString()}]` +
        ": " +
        (typeof value === "string" ? value : JSON.stringify(value)),
      ...r,
    ]);
  }

  onMount(async () => {
    ipfs_id_unlisten = await listen("ipfs-id", onIpfsId);
  });

  onDestroy(() => {
    if (ipfs_id_unlisten) {
      ipfs_id_unlisten();
    }
  });
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
  <svelte:component this={selected_view.component} />
</Content>
