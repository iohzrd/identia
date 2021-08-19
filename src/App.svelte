<script>
  import {
    Header,
    SideNav,
    SideNavItems,
    SideNavMenu,
    SideNavMenuItem,
    SideNavLink,
    SideNavDivider,
    SkipToContent,
    Content,
  } from "carbon-components-svelte";

  import { listen, emit } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";
  import { writable } from "svelte/store";
  // import { create } from "ipfs-http-client";

  // export let onMessage;
  let isSideNavOpen = false;
  let unlisten;
  let count = 1;
  // let ipfs;
  // let ipfs_id = {};
  let ipfs_id = "";
  let responses = writable([]);

  // the `$:` means 're-run whenever these values change'
  $: doubled = count * 2;
  $: quadrupled = doubled * 2;

  onMount(async () => {
    unlisten = await listen("rust-event", onMessage);

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
    if (unlisten) {
      unlisten();
    }
  });

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
      .then(onMessage)
      .catch(onMessage);
  }

  function ipfsID() {
    invoke("ipfs_id").then(onIpfsId).catch(onMessage);
  }

  function emitEvent() {
    emit("ipfs-id", "this is the payload string");
  }

  function onMessage(value) {
    console.log("onMessage");
    console.log(value);
    responses.update((r) => [
      `[${new Date().toLocaleTimeString()}]` +
        ": " +
        (typeof value === "string" ? value : JSON.stringify(value)),
      ...r,
    ]);
  }

  function onIpfsId(value) {
    console.log("onIpfsId");
    console.log(value);
    ipfs_id = value;
  }
</script>

<Header
  persistentHamburgerMenu={true}
  company="IBM"
  platformName="Carbon Svelte"
  bind:isSideNavOpen
>
  <div slot="skip-to-content">
    <SkipToContent />
  </div>
</Header>

<SideNav bind:isOpen={isSideNavOpen}>
  <SideNavItems>
    <SideNavLink text="Link 1" />
    <SideNavLink text="Link 2" />
    <SideNavLink text="Link 3" />
    <SideNavMenu text="Menu">
      <SideNavMenuItem href="/" text="Link 1" />
      <SideNavMenuItem href="/" text="Link 2" />
      <SideNavMenuItem href="/" text="Link 3" />
    </SideNavMenu>
    <SideNavDivider />
    <SideNavLink text="Link 4" />
  </SideNavItems>
</SideNav>

<Content>
  <div>
    <h3>
      Hello, {ipfs_id}
    </h3>

    <p>{count} * 2 = {doubled}</p>
    <p>{doubled} * 2 = {quadrupled}</p>

    <button on:click={handleClick}>
      Count: {count}
    </button>

    <div>{window.location}</div>
    <div>{window.location.href}</div>

    <button class="button" id="log" on:click={log}>Call Log API</button>
    <button class="button" id="id" on:click={performRequest}>
      Call Request (async) API
    </button>
    <button class="button" id="request" on:click={ipfsID}> ipfs_id </button>
    <button class="button" id="event" on:click={emitEvent}>
      Send event to Rust
    </button>
  </div>
</Content>
