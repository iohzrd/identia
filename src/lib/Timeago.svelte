<script lang="ts">
  import { format } from "timeago.js";
  import { onMount, onDestroy } from "svelte";

  interface Props {
    timestamp: number;
  }

  let { timestamp }: Props = $props();

  let timer: ReturnType<typeof setTimeout>;
  let timeout_time = 1000;
  let timeago: string = $state(format(timestamp));
  let datetime: string = new Date(timestamp).toLocaleString();

  function newTimeout() {
    timeago = format(timestamp);
    let delta = new Date().getTime() - timestamp;
    if (delta < 60 * 1000) {
      // less than a minue, update once a second
      timeout_time = 1000;
    } else if (delta < 60 * 60 * 1000) {
      // less than an hour, update once a minute
      timeout_time = 60 * 1000;
    } else {
      // update once an hour
      timeout_time = 60 * 60 * 1000;
    }
    timer = setTimeout(newTimeout, timeout_time);
  }

  onMount(async () => {
    timer = setTimeout(newTimeout, timeout_time);
  });

  onDestroy(() => {
    clearTimeout(timer);
  });
</script>

{timeago} ({datetime})
