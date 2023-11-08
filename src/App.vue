<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
document.addEventListener('DOMContentLoaded', () => {
  // This will wait for the window to load, but you could
  // run this function on whatever trigger you want
  setTimeout(() => {
    invoke('close_splashscreen')
  }, 2000);
})
</script>

<template>
  <div class="container">
    <router-view></router-view>
  </div>
</template>

<style scoped>
.container {
  width: 100%;
  height: 100%;
}
</style>
