<script lang="ts">
  import * as monaco from 'monaco-editor';
  import { HSplitPane, VSplitPane } from 'svelte-split-pane';
  import { onMount } from 'svelte';

  const languageConfig = {
    id: 73,
    name: 'rust',
  };

  let container,
    code = 'fn main() {\n  println!("Hello, world!"); \n}',
    error = '';

  onMount(() => {
    let editor = monaco.editor.create(container, {
      value: code,
      language: languageConfig.name,
      fontSize: '16px',
      automaticLayout: true,
      overviewRulerLanes: 0,
    });

    editor.onDidChangeModelContent(() => {
      code = editor.getValue();
    });
  });

  let query = '',
    data = null;

  async function handleInput(event) {
    try {
      const response = await fetch(
        'http://localhost:8000/search?query=' + event.target.value
      );
      data = await response.json();
    } catch (err) {
      error = err.toString();
    }
  }

  let currStatus = {};

  async function runCode() {
    console.log(`Running code: ${code}...`);

    try {
      const response = await fetch('/judge/submissions', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          source_code: code,
          language_id: languageConfig.id,
          stdin: '',
          cpu_time_limit: 2,
          wall_time_limit: 5,
          memory_limit: 128000,
        }),
      });

      const id = (await response.json()).token;

      console.log(id);

      let status = { description: 'Queue' };

      while (
        status.description !== 'Accepted' &&
        status.description !== 'Compilation Error'
      ) {
        const result = await fetch(`/judge/submissions/${id}`);
        const data = await result.json();
        console.log(data);
        status = data.status;
        console.log(status);
        await new Promise((resolve) => setTimeout(resolve, 1000));
      }

      currStatus = status;
    } catch (error) {
      console.error(error);
    }
  }
</script>

<main class="bg-slate-50 h-screen overflow-hidden">
  <h1 class="font-bold text-md text-center p-1">
    <a href="/">rustground ????<a /></a>
  </h1>
  <HSplitPane minLeftPaneSize="25%" minRightPaneSize="25%">
    <div slot="left">
      <div class="items-center m-2">
        <label for="simple-search" class="sr-only">Search</label>
        <div class="relative w-full">
          <div
            class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none"
          >
            <svg
              aria-hidden="true"
              class="w-5 h-5 text-gray-500 dark:text-gray-400"
              fill="currentColor"
              viewBox="0 0 20 20"
              xmlns="http://www.w3.org/2000/svg"
              ><path
                fill-rule="evenodd"
                d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
                clip-rule="evenodd"
              /></svg
            >
          </div>
          <input
            type="text"
            id="simple-search"
            class="w-full pl-10 p-2.5 rounded-lg"
            placeholder="Search crates"
            bind:value={query}
            on:input={handleInput}
          />
        </div>
        {#if error} <p>{error}</p> {/if}
        {#if data}
          {#each data.hits.hits as item}
            <div class="border border-blue-100 mt-2 rounded-lg p-2">
              <p class="font-bold">{item._id}</p>
              <p>{item._source.description}</p>
            </div>
          {/each}
        {/if}
        <button
          class="bg-blue-100 text-black font-bold mt-2 py-2 px-4 rounded-lg w-full"
          on:click={runCode}
        >
          Run
        </button>
      </div>
    </div>
    <div slot="right" class="h-full">
      <VSplitPane minDownPaneSize="25%" minTopPaneSize="25%">
        <div slot="top" bind:this={container} class="h-full w-full" />
        <div slot="down">{JSON.stringify(currStatus)}</div>
      </VSplitPane>
    </div>
  </HSplitPane>
</main>
