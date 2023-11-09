<template>
  <div class="container">
    <div>
      <a-textarea
        v-model:value="Store.noteName"
        placeholder="笔记名称"
        auto-size
      />
    </div>
      <Editor />
  </div>
</template>

<script setup lang="ts">
import Editor from "../../components/Editor/index.vue";
import { useStore } from '../../store/index'
// import { ref } from "vue";
const Store = useStore()
</script>

<style lang="less" scoped>
.container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}
</style>
