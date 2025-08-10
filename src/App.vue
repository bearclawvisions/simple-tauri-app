<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

const title = ref('tauri-cv')
const isMaximized = ref(false)

const minimizeWindow = async () => {
  await invoke('minimize_window')
}

const maximizeWindow = async () => {
  await invoke('maximize_window')
  isMaximized.value = true
}

const unmaximizeWindow = async () => {
  await invoke('unmaximize_window')
  isMaximized.value = false
}

const toggleMaximize = async () => {
  if (isMaximized.value) {
    await unmaximizeWindow()
  } else {
    await maximizeWindow()
  }
}

const closeWindow = async () => {
  await invoke('close_window')
}

onMounted(async () => {
  isMaximized.value = await invoke('is_window_maximized')
})

</script>

<template>
  <div class="titlebar">
    <div class="titlebar-content" data-tauri-drag-region>
      <span class="title">{{ title }}</span>
    </div>
    <div class="titlebar-controls">
      <button @click="minimizeWindow" class="titlebar-button minimize">
        <span>−</span>
      </button>
      <button @click="toggleMaximize" class="titlebar-button maximize">
        <span>{{ isMaximized ? '◱' : '□' }}</span>
      </button>
      <button @click="closeWindow" class="titlebar-button close">
        <span>x</span>
      </button>
    </div>
  </div>

  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>

    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="./assets/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="./assets/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  top: 30px; /* Start below titlebar */
  left: 0;
  right: 0;
  bottom: 0;
  overflow-y: auto; /* This container will scroll */
  padding: 30px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  box-sizing: border-box;
}

/* Custom scrollbar styling (optional) */
.container::-webkit-scrollbar {
  width: 8px;
}

.container::-webkit-scrollbar-track {
  background: #f1f1f1;
}

.container::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

.container::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}



.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

.titlebar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 30px;
  background-color: #2f2f2f;
  color: white;
  user-select: none;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
}

.titlebar-content {
  flex: 1;
  display: flex;
  align-items: center;
  padding-left: 10px;
}

.title {
  font-size: 14px;
  font-weight: 500;
}

.titlebar-controls {
  display: flex;
}

.titlebar-button {
  width: 46px;
  height: 30px;
  background: transparent;
  border: none;
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  transition: background-color 0.2s;
}

.titlebar-button:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.titlebar-button.close:hover {
  background-color: #e81123;
}

.titlebar-button span {
  pointer-events: none;
}

</style>