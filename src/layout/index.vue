<template>
  <div class="container">
    <Tree />
    <div class="content">
      <router-view></router-view>
    </div>
  </div>
</template>

<script setup lang="ts">
import Tree from '../components/Tree/index.vue'
</script>

<style lang="less" scoped>
.container {
  display: flex;
  .content {
    flex: 1;
    padding: 10px;
  }
}
</style>
