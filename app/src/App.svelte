<script>
  import { invoke } from '@tauri-apps/api/core';
  import svelteLogo from './assets/svelte.svg'
  import viteLogo from '/vite.svg'
  import Counter from './lib/Counter.svelte'
  
  let greetingMessage = '';
  let selectedPath = '';

  async function greet() {
    try {
      greetingMessage = await invoke('greet', { name: 'Tauri V2' });
    } catch (error) {
      console.error('Error invoking greet:', error);
      greetingMessage = 'Error: ' + error.message;
    }
  }

  async function selectFolder() {
    try {
      // 暂时注释掉
      // const selected = await open({
      //   directory: true,
      //   multiple: false
      // });
      // if (selected) {
      //   selectedPath = selected;
      // }
      selectedPath = "功能暂时不可用 - API 需要更新";
    } catch (error) {
      console.error('Error selecting folder:', error);
    }
  }
</script>

<main>
  <h1>Mirutas - Tauri 测试</h1>
  <div>
    <a href="https://vite.dev" target="_blank" rel="noreferrer">
      <img src={viteLogo} class="logo" alt="Vite Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank" rel="noreferrer">
      <img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <div class="card">
    <button on:click={greet}>Greet from Rust</button>
    <p>{greetingMessage}</p>
  </div>

  <div class="folder-selection">
    <button on:click={selectFolder}>Select Folder</button>
    <p>Selected path: {selectedPath || 'None'}</p>
  </div>
</main>

<style>
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .read-the-docs {
    color: #888;
  }

  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
    text-align: center;
    gap: 2rem;
  }

  h1 {
    color: #0f0f0f;
    font-size: 3.2em;
    line-height: 1.1;
  }

  .card {
    padding: 2em;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    background-color: #f9f9f9;
    cursor: pointer;
    transition: border-color 0.25s;
  }
  
  button:hover {
    border-color: #646cff;
  }
  
  button:focus,
  button:focus-visible {
    outline: 4px auto -webkit-focus-ring-color;
  }

  .folder-selection {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }
</style>
