<script>
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

  import Identity from "./components/Identity.svelte";
  import Feed from "./components/Feed.svelte";
  import Settings from "./components/Settings.svelte";

  import { listen, emit } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";
  // import { writable } from "svelte/store";
  // import { create } from "ipfs-http-client";

  // export let onIpfsId;
  let isSideNavOpen = false;
  let ipfs_id_unlisten;
  let count = 1;
  // let ipfs;
  let ipfs_id = "";
  // let responses = writable([]);

  // the `$:` means 're-run whenever these values change'
  $: doubled = count * 2;
  $: quadrupled = doubled * 2;

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
  ];
  let selected_view = views[0];

  function select(view) {
    selected_view = view;
  }

  function handleClick() {
    count += 1;
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
    invoke("request_test_obj").then(onTestObj).catch(onTestObj);
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

    // try {
    //   console.log(navigator.userAgent);
    //   // navigator.userAgent = "Tauri";
    //   console.log(navigator.userAgent);
    //   ipfs = await create("/ip4/127.0.0.1/tcp/5001");
    //   console.log("ipfs in svelte!");

    //   ipfs_id = await ipfs.id();
    //   console.log(ipfs_id);
    //   id = ipfs_id.id;
    // } catch (error) {
    //   console.log(error);
    // }
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
  <button class="button" id="id" on:click={requestTestObj}>
    Request obj (async)
  </button>

  <svelte:component this={selected_view.component} />
</Content>
