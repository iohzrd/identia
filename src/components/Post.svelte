<script lang="ts">
  import type { Post, PostResponse } from "../types.type";
  import { Buffer } from "buffer/index";
  import { ClickableTile } from "carbon-components-svelte";
  import { Link } from "carbon-components-svelte";
  import { Player, Video, DefaultUi } from "@vime/svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";

  let player: Player;
  export let cid: String;
  export let postResponse: PostResponse;
  export let includeFrom: Boolean = true;
  $: post = postResponse.post;
  let data;

  function onPostResponseObject(pr: PostResponse) {
    console.log("onPostObject");
    console.log(pr);
    if (pr && pr.cid && pr.post) {
      postResponse = pr;
    }
  }

  function onBinaryData(buffer) {
    console.log("onBinaryData");
    const buf = Buffer.from(buffer);
    var blob = new Blob([buf], { type: "image/png" });
    var urlCreator = window.URL || window.webkitURL;
    data = urlCreator.createObjectURL(blob);
  }

  function getPostFromCid() {
    invoke("get_post_ipfs", {
      cid: cid,
    })
      .then(onPostResponseObject)
      .catch(onPostResponseObject);
  }

  function getBinaryData(cid) {
    invoke("get_binary_data", {
      cid: cid,
    })
      .then(onBinaryData)
      .catch(onBinaryData);
  }

  onMount(async () => {
    console.log("Post");
    console.log(postResponse);
    // getBinaryData("ipfs/QmTyTgT6N1DV4ARDwvrX7ucVuiUyTnDQpCKAu8zwMBgPfE");
    getBinaryData("QmUXkyqR8NgrxvDTXgFzAe87kGG58a1K4miVGgyxN37gmE/phi.png");

    // for await (const file of files) {
    //     // var buf = Buffer.concat(await all(ipfs.cat(file.path)));
    //     let bufs = [];
    //     for await (const buf of ipfs.cat(file.path)) {
    //       bufs.push(buf);
    //     }
    //     const buf = Buffer.concat(bufs);
    //     const fType = await window.ipc.getFileTypeFromBuffer(buf);
    //     if (fType && fType.mime) {
    //       var blob = new Blob([buf], { type: fType.mime });
    //       var urlCreator = window.URL || window.webkitURL;
    //       var data = urlCreator.createObjectURL(blob);
    //     } else if (file && file.name.includes(".md")) {
    //       blob = new Blob([buf]);
    //       data = await blob.text();
    //     }
    //     const fileObj = {
    //       ...file,
    //       ...fType,
    //       data,
    //     };
    //     this.fileObjs.push(fileObj);
    //   }

    if (!postResponse) {
      getPostFromCid();
    }
  });

  onDestroy(() => {});

  const onPlaybackReady = () => {
    // ...
  };
</script>

<ClickableTile>
  {#if post}
    {#if post && post.body}
      <div>
        <!-- {@html post.body.replace(/\n/g, "<br>")} -->
        {post.body}
      </div>
    {/if}
    {#if data}
      <div>
        img
        <img src={data} alt="" />
      </div>
      <!-- <div id="container">
        <Player bind:this={player}>
          <Video crossOrigin="" poster="https://media.vimejs.com/poster.png">
            <source
              data-src="https://media.vimejs.com/720p.mp4"
              type="video/mp4"
            />
          </Video>

          <DefaultUi>
            <TapSidesToSeek />
          </DefaultUi>
        </Player>
      </div> -->
    {/if}
    {#if post && post.publisher && includeFrom}
      <div>
        publisher: <Link href="#/identity/{post.publisher}">
          {post.publisher}
        </Link>
      </div>
    {/if}
    {#if post && post.timestamp}
      <div>
        {post.timestamp}
      </div>
    {/if}
  {/if}
</ClickableTile>
