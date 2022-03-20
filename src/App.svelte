<script lang="ts">
  import {
    Button,
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
  import Database from "tauri-plugin-sql-api";
  import Feed from "./components/Feed.svelte";
  import Identity from "./components/Identity.svelte";
  import Router from "svelte-spa-router";
  import { create } from "ipfs-http-client/index";
  // import { followPublisher } from "./Core.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { location } from "svelte-spa-router";
  import { multihash } from "is-ipfs";
  import { onMount, onDestroy } from "svelte";

  let ipfs;
  let ipfs_info;

  let follow_modal_open = false;
  let ipfs_id: string;
  let publisher_to_follow: string = "";
  $: publisher_to_follow_invalid = !multihash(publisher_to_follow);
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
    let follow_success = await invoke("follow_publisher", {
      publisher: publisher_to_follow.trim(),
    });
    if (follow_success) {
      clearFollowModal();
    }
  }

  function clearFollowModal() {
    follow_waiting = false;
    publisher_to_follow = "";
  }

  onMount(async () => {
    ipfs = await create("/ip4/127.0.0.1/tcp/5001");
    ipfs_info = await ipfs.id();
    console.log(ipfs_info);
    ipfs_id = ipfs_info.id;
    // const db = await Database.load(`sqlite:${ipfs_id}.db`);
    const db = await Database.load(`sqlite:sqlite.db`);

    try {
      const insert = await db.execute(
        "INSERT INTO identities (cid,avatar,description,display_name,following,meta,posts,publisher,timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        [
          "test",
          "test",
          "test",
          "test",
          ["12D3KooWHxU85q4JWsDXq4ZHjBCdjHHGL9wnMtqBMMgArkn6xcyz"],
          { BTC: "1234", twitter: { url: "https://twitter.com/iohzrd" } },
          ["post1", "post2", "post3"],
          "test",
          0,
        ]
      );
      console.log(insert);
    } catch (error) {
      console.log(error);
    }

    const select = await db.select("SELECT * from identities");
    console.log(select);
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
        on:click={() => (follow_modal_open = true)}
      />
    </HeaderUtilities>
  </Header>

  <Modal
    bind:open={follow_modal_open}
    modalHeading="Follow publisher"
    on:close={clearFollowModal}
    on:open
    passiveModal
    size="lg"
  >
    <TextInput
      invalid={publisher_to_follow_invalid}
      invalidText="Invalid IPNS id. Please try another."
      labelText="publisher to follow"
      placeholder="12D3KooW..."
      disabled={follow_waiting}
      bind:value={publisher_to_follow}
    />
    {#if follow_waiting}
      <ProgressBar helperText="Please wait..." />
    {:else}
      <Button disabled={publisher_to_follow_invalid} on:click={followPublisher}
        >Confirm</Button
      >
    {/if}
  </Modal>

  <Content>
    <Router {routes} />
  </Content>
{:else}
  <Loading />
{/if}
