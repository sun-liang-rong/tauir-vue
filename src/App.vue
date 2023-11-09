<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
document.addEventListener('DOMContentLoaded', () => {
  init()
  // This will wait for the window to load, but you could
  // run this function on whatever trigger you want
  setTimeout(() => {
    invoke('close_splashscreen')
    
  }, 2000);
})
//查看是不是又data文件
const look = async () => {
  try {
    let a = await invoke('check_directory_exists', {path: 'data'});
    console.log('文件夹创建成功！', a);
    if(!a) {
      //没有就创建一个data的文件夹存放所有的md笔记
      createDir()
    }
  } catch (error) {
    console.error('文件夹创建失败：', error);
  }
}
const createDir = async () => {
  try {
    await invoke('my_read_file', {path: 'data'});
    // lookSubDir()
    await invoke('my_read_file', {path: 'data/默认分类'});
    console.log('文件夹创建成功！');
  } catch (error) {
    console.error('文件夹创建失败：', error);
  }
}
// const lookSubDir = async () => {
//   try {
//     await invoke('check_subdirectories_exist', {path: '默认分类'})
//   } catch (error) {
//     console.error('check_subdirectories_exist', error);
//   }
// }
const init = () => {
  look()
}
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
