<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

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
</template>