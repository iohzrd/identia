<script>
  let count = 1;
  $: doubled = count * 2;
  $: quadrupled = doubled * 2;

  function handleClick() {
    count += 1;
  }
</script>

<p>{count} * 2 = {doubled}</p>
<p>{doubled} * 2 = {quadrupled}</p>

<button on:click={handleClick}>
  Count: {count}
</button>

<div>{window.location}</div>
