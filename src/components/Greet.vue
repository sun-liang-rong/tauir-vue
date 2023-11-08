<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");
const helloMsg = ref("");
const helloName = ref("");
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
  console.log(greetMsg)
}


async function hello() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  helloMsg.value = await invoke("hello", {helloMsg: helloMsg.value});
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>
  <form class="row" @submit.prevent="hello">
    <input id="greet-input" v-model="helloName" placeholder="hello..." />
    <button type="submit">hello</button>
  </form>

  <div style="margin-top: 40px;">
    <p>{{ greetMsg }}</p>
    <p>{{ helloMsg }}</p>
  </div>
</template>
