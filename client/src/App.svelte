<script>
  let query = '';

  let data = null;

  async function handleInput(event) {
    const response = await fetch('http://localhost:8000/search?query=' + event.target.value)
    data = await response.json();
  }
</script>

<main class='m-2'>
  <h1 class='font-bold'>rustground 🛝</h1>

  <label>
    search: <input class="border border-sky-500" type="text" bind:value={query} on:input={handleInput}>
  </label>

  {#if data}
    <!-- <p>{JSON.stringify(data.results.hits.hits)}</p> -->
    {#each data.results.hits.hits as item}
      <div class="border border-sky-500 m-2">
      <p>Name: {item._id}</p>
      <p>Description: {item._source.description}
      </div>
    {/each}
    <!-- <p>{JSON.stringify(data)}</p> -->
  {/if}
</main>
