<script>
  import { onMount } from "svelte";
  import { create } from "ipfs-http-client";

  let count = 1;
  let ipfs;
  let ipfs_id = {};
  let id = "";

  // the `$:` means 're-run whenever these values change'
  $: doubled = count * 2;
  $: quadrupled = doubled * 2;

  function handleClick() {
    count += 1;
  }

  onMount(async () => {
    window.location.href = "file://index.html";
    try {
      console.log(navigator.userAgent);
      // navigator.userAgent = "Tauri";
      console.log(navigator.userAgent);
      ipfs = await create("/ip4/127.0.0.1/tcp/5001");
      console.log("ipfs in svelte!");

      ipfs_id = await ipfs.id();
      console.log(ipfs_id);
      id = ipfs_id.id;
    } catch (error) {
      console.log(error);
    }
  });
</script>

<h3>
  Hello, {id}
</h3>

<p>{count} * 2 = {doubled}</p>
<p>{doubled} * 2 = {quadrupled}</p>

<button on:click={handleClick}>
  Count: {count}
</button>

<div>{window.location}</div>
<div>{window.location.href}</div>
