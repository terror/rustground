<script>
  import * as monaco from 'monaco-editor';

  import { onMount } from 'svelte';

  let container;

  let code = 'fn main() {\n  println!("Hello, world!"); \n}';

  let error = '';

  onMount(() => {
    let editor = monaco.editor.create(container, {
      value: code,
      language: 'rust',
      theme: 'vs-dark',
    });

    editor.onDidChangeModelContent(() => {
      code = editor.getValue();
    });
  });

  let query = '';

  let data = null;

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

  async function runCode() {
    console.log(`Running code: ${code}...`);

    try {
      const response = await fetch('/judge/submissions', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          source_code: code,
          language_id: 73,
          stdin: '',
          expected_output: '',
          cpu_time_limit: 2,
          wall_time_limit: 5,
          memory_limit: 128000,
        }),
      });

      const submission = await response.json();
      const submissionId = submission.token;

      console.log(submissionId);

      let status = { description: 'Queue' };

      while (
        status.description !== 'Accepted' &&
        status.description !== 'Compilation Error'
      ) {
        const result = await fetch(`/judge/submissions/${submissionId}`);
        const data = await result.json();
        console.log(data);
        status = data.status;
        console.log(status);
        await new Promise((resolve) => setTimeout(resolve, 1000)); // Wait for 1 second before checking again
      }

      if (status.description === 'Accepted') {
        const output = status.stdout;
        console.log(output);
      } else {
        const error = status.compile_output || status.stderr;
        console.error(error);
      }
    } catch (error) {
      console.error(error);
    }
  }
</script>

<main class="m-2 h-screen">
  <h1 class="font-bold">rustground 🛝</h1>

  <label>
    search crates: <input
      class="border border-sky-500"
      type="text"
      bind:value={query}
      on:input={handleInput}
    />
  </label>

  {#if data}
    <!-- <p>{JSON.stringify(data.results.hits.hits)}</p> -->
    {#each data.results.hits.hits as item}
      <div class="border border-sky-500 m-2">
        <p>Name: {item._id}</p>
        <p>Description: {item._source.description}</p>
      </div>
    {/each}
    <!-- <p>{JSON.stringify(data)}</p> -->
  {/if}

  {#if error}
    <p>{error}</p>
  {/if}

  <div bind:this={container} class="h-3/6 mt-2" />

  <button
    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mt-2"
    on:click={runCode}
  >
    Run
  </button>
</main>
