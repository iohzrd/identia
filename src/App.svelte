<script lang="ts">
  import {
    Button,
    Content,
    Grid,
    Header,
    HeaderActionLink,
    HeaderGlobalAction,
    HeaderSearch,
    HeaderUtilities,
    Loading,
    Modal,
    ProgressBar,
    SideNav,
    SideNavDivider,
    SideNavItems,
    SideNavLink,
    SkipToContent,
    TextInput,
  } from "carbon-components-svelte";
  import FeedComponent from "./components/Feed.svelte";
  import IdentityComponent from "./components/Identity.svelte";
  import Router from "svelte-spa-router";
  import WebFeedComponent from "./components/WebFeed.svelte";
  import WebPublisherComponent from "./components/WebPublisher.svelte";
  import type { IDResult } from "ipfs-core-types/src/root";
  import type { Message } from "ipfs-http-client/pubsub/subscribe";
  import { Add, UserAvatarFilled } from "carbon-icons-svelte";
  import { followPublisher, getIdentity, ipfs } from "./core";
  import { getTauriVersion, getVersion } from "@tauri-apps/api/app";
  import { location } from "svelte-spa-router";
  import { multihash } from "is-ipfs";
  import { onMount, onDestroy } from "svelte";
  import { peerIdFromPeerId } from "@libp2p/peer-id";

  let isSideNavOpen = false;

  let app_version: string;
  let ipfs_id: string;
  let ipfs_info: IDResult;
  let ipfs_version: string;
  let tauri_version: string;

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
      label: "Web Feed",
      path: "/webfeed/",
    },
  ];

  const routes = {
    "/:publisher?": FeedComponent,
    "/identity/:publisher?": IdentityComponent,
    "/webfeed/:publisher?": WebFeedComponent,
    "/webpublisher/:b64_publisher?": WebPublisherComponent,
    // "*": NotFound,
  };

  const data = [];
  let ref = null;
  let active = false;
  let value = "";
  let selectedResultIndex = 0;
  let events = [];

  $: lowerCaseValue = value.toLowerCase();
  $: results =
    value.length > 0
      ? data.filter((item) => {
          return (
            item.text.toLowerCase().includes(lowerCaseValue) ||
            item.description.includes(lowerCaseValue)
          );
        })
      : [];

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

  function on_pubsub_message(m) {
    console.log(`on_pubsub_message`, m);
    let d = m.data;
    // 123 == "{" 125 == "}"
    if (d.length > 0 && d[0] == 123 && d[d.length - 1] == 125) {
      let decoded = new TextDecoder().decode(d);
      let parsed = JSON.parse(decoded);
      console.log(`on_pubsub_message`, parsed);
    }
    const id = peerIdFromPeerId(m.from);
    console.log(id);
  }

  onMount(async () => {
    app_version = await getVersion();
    tauri_version = await getTauriVersion();
    ipfs_info = await ipfs.id();
    ipfs_version = ipfs_info.agentVersion.split("/")[1];
    console.log(ipfs_info);
    await getIdentity(ipfs_info.id.toString());
    ipfs_id = ipfs_info.id.toString();
    console.log(await ipfs.pubsub.ls());
    await ipfs.pubsub.subscribe(ipfs_id, on_pubsub_message);
    await ipfs.pubsub.publish(
      ipfs_id,
      new TextEncoder().encode(JSON.stringify({ blah: "blah" }))
    );
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
      <HeaderSearch
        bind:ref
        bind:active
        bind:value
        bind:selectedResultIndex
        placeholder="Search services"
        {results}
        on:active={() => {
          events = [...events, { type: "active" }];
        }}
        on:inactive={() => {
          events = [...events, { type: "inactive" }];
        }}
        on:clear={() => {
          events = [...events, { type: "clear" }];
        }}
        on:select={(e) => {
          events = [...events, { type: "select", ...e.detail }];
        }}
      />
      <HeaderActionLink href="#/identity/{ipfs_id}" icon={UserAvatarFilled} />
      <HeaderGlobalAction
        aria-label="Follow new identity"
        icon={Add}
        on:click={() => (follow_modal_open = !follow_modal_open)}
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
      <SideNavDivider />
      <SideNavLink href="https://github.com/iohzrd/identia" target="_blank">
        identia: v{app_version}
      </SideNavLink>
      <SideNavLink href="https://github.com/ipfs/go-ipfs" target="_blank">
        ipfs: v{ipfs_version}
      </SideNavLink>
      <SideNavLink href="https://github.com/tauri-apps/tauri" target="_blank">
        tauri: v{tauri_version}
      </SideNavLink>
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
