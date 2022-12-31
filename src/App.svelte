<script lang="ts">
  import {
    Button,
    Content,
    Grid,
    Header,
    HeaderActionLink,
    HeaderGlobalAction,
    HeaderUtilities,
    Loading,
    Modal,
    ProgressBar,
    SideNav,
    SideNavDivider,
    SideNavItems,
    SideNavLink,
    SideNavMenu,
    SideNavMenuItem,
    SkipToContent,
    TextInput,
  } from "carbon-components-svelte";
  import FeedComponent from "./components/Feed.svelte";
  import IdentityComponent from "./components/Identity.svelte";
  import Router from "svelte-spa-router";
  import TopicFeedComponent from "./components/TopicFeed.svelte";
  import WebFeedComponent from "./components/WebFeed.svelte";
  import WebPublisherComponent from "./components/WebPublisher.svelte";
  import type { IDResult } from "ipfs-core-types/src/root";
  import { Add, UserAvatarFilled } from "carbon-icons-svelte";
  import { followPublisher, getIdentity, ipfs } from "./core";
  import { getTauriVersion, getVersion } from "@tauri-apps/api/app";
  import {
    deleteTopicFromDB,
    getTopicsFromDB,
    globalPubsubHandler,
    insertTopicIntoDB,
  } from "./pubsub";
  import { location } from "svelte-spa-router";
  import { multihash } from "is-ipfs";
  import { onMount, onDestroy } from "svelte";

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

  let topic_modal_open = false;
  let topic_to_follow: string = "";

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
    "/topicfeed/:topic?": TopicFeedComponent,
    // "*": NotFound,
  };

  let subs: string[] = [];

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

  async function followTopic() {
    console.log(followTopic);
    await insertTopicIntoDB(topic_to_follow);
    subs = await getTopicsFromDB();
    console.log(subs);
    topic_modal_open = false;
  }

  async function unfollowTopic(topic) {
    await deleteTopicFromDB(topic);
    subs = await getTopicsFromDB();
  }

  onMount(async () => {
    app_version = await getVersion();
    tauri_version = await getTauriVersion();
    ipfs_info = await ipfs.id();
    ipfs_version = ipfs_info.agentVersion.split("/")[1];
    console.log(ipfs_info);
    await getIdentity(ipfs_info.id.toString());
    ipfs_id = ipfs_info.id.toString();
    subs = await getTopicsFromDB();
    for await (const topic of [ipfs_id, ...subs]) {
      await ipfs.pubsub.subscribe(topic, globalPubsubHandler);
    }
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

      <SideNavMenu text="Topic Feeds" expanded>
        {#each subs as topic}
          <SideNavLink
            href="#/topicfeed/{topic}"
            text="/{topic}/"
            isSelected={$location === "/topicfeed/" + topic}
          />
        {/each}
        <SideNavMenuItem
          text="Add new Topic"
          on:click={() => (topic_modal_open = !topic_modal_open)}
        />
      </SideNavMenu>

      <div style="bottom: 0; position: absolute; width: 100%;">
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
      </div>
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

  <Modal
    bind:open={topic_modal_open}
    modalHeading="Add topic"
    on:close={() => (topic_to_follow = "")}
    on:open
    passiveModal
    size="lg"
  >
    <TextInput
      invalid={false}
      invalidText=""
      labelText="topic to follow"
      placeholder="pol"
      disabled={false}
      bind:value={topic_to_follow}
    />
    <Button disabled={subs.includes(topic_to_follow)} on:click={followTopic}
      >Follow Topic</Button
    >
  </Modal>
{:else}
  <Loading />
{/if}
