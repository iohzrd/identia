<script lang="ts">
  import "@splidejs/splide/dist/css/splide.min.css";
  // import "carbon-components-svelte/css/all.css";
  import "carbon-components-svelte/css/g100.css";
  import {
    Button,
    ButtonSet,
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
  import TrashCan from "carbon-icons-svelte/lib/TrashCan.svelte";
  import type { IDResult } from "ipfs-core-types/src/root";
  import { Add, UserAvatarFilled } from "carbon-icons-svelte";
  import {
    followPublisher,
    getIdentity,
    ipfs,
    publishIdentity,
  } from "$lib/core";
  import { getTauriVersion, getVersion } from "@tauri-apps/api/app";
  import {
    deleteTopicFromDB,
    getTopicsFromDB,
    globalPubsubHandler,
    insertTopicIntoDB,
  } from "$lib/pubsub";
  import { goto } from "$app/navigation";
  import { multihash } from "is-ipfs";
  import { onMount, onDestroy } from "svelte";
  import { page } from "$app/stores";

  let isSideNavOpen = false;

  let ipfs_id: string;
  let ipfs_info: IDResult;
  let republishInterval: any;

  let app_version: string;
  let ipfs_version: string;
  let tauri_version: string;

  let follow_modal_open: boolean = false;
  let follow_waiting: boolean = false;
  let publisher_to_follow: string = "";
  $: publisher_invalid = !multihash(publisher_to_follow);

  let topic_modal_open = false;
  let topic_to_follow: string = "";
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
    goto(`/topicfeed/${topic_to_follow}`);
  }

  async function unfollowTopic(topic: string) {
    await deleteTopicFromDB(topic);
    subs = await getTopicsFromDB();
  }

  async function republish() {
    let identity = await getIdentity(ipfs_id);
    let next = new Date().getTime() - 1000 * 60 * 60 * 12;
    if (identity.timestamp < next) {
      console.log("republishing identity...");
      await publishIdentity();
    }
  }

  onMount(async () => {
    app_version = await getVersion();
    tauri_version = await getTauriVersion();
    ipfs_info = await ipfs.id();
    ipfs_version = ipfs_info.agentVersion.split("/")[1];
    console.log(ipfs_info);
    ipfs_id = ipfs_info.id.toString();
    await getIdentity(ipfs_id); // init identity if necessary...
    subs = await getTopicsFromDB();
    for await (const topic of [ipfs_id, ...subs]) {
      await ipfs.pubsub.subscribe(topic, globalPubsubHandler);
    }

    // periodically republish...
    // should refactor to republish based on previous
    await republish();
    republishInterval = setInterval(republish, 1000 * 60 * 15);
  });

  onDestroy(() => {
    clearInterval(republishInterval);
  });
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
      <HeaderActionLink href="/identity/{ipfs_id}" icon={UserAvatarFilled} />
      <HeaderGlobalAction
        aria-label="Follow new identity"
        icon={Add}
        on:click={() => (follow_modal_open = !follow_modal_open)}
      />
    </HeaderUtilities>
  </Header>

  <SideNav bind:isOpen={isSideNavOpen}>
    <SideNavItems>
      <SideNavLink
        href="/"
        text="Feed"
        isSelected={$page.url.pathname === "/"}
      />
      <SideNavLink
        href="/identity/{ipfs_id}"
        text="Identity"
        isSelected={$page.url.pathname.includes("/identity/")}
      />
      <SideNavLink
        href="/webfeed"
        text="Web Feed"
        isSelected={$page.url.pathname.includes("/webfeed/")}
      />
      <SideNavMenu text="Topic Feeds" expanded>
        {#each subs as topic}
          <ButtonSet>
            <SideNavLink
              href="/topicfeed/{topic}"
              isSelected={$page.url.pathname === "/topicfeed/" + topic}
              style="width: 20em; overflow: hidden"
              text="/{topic}/"
            />
            {#if $page.url.pathname === "/topicfeed/" + topic}
              <Button
                icon={TrashCan}
                kind="danger-tertiary"
                size="small"
                style="position: absolute; right: 0; width: 12.5%;"
                tooltipPosition="right"
                on:click={() => unfollowTopic(topic)}
              />
            {/if}
          </ButtonSet>
        {/each}
        {#if topic_modal_open}
          <SideNavMenuItem text="/{topic_to_follow}/" />
        {/if}
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
      <slot />
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
    <Button
      disabled={!topic_to_follow || subs.includes(topic_to_follow)}
      on:click={followTopic}>Follow Topic</Button
    >
  </Modal>
{:else}
  <Loading />
{/if}
