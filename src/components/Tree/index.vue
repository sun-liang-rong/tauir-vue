<template>
  <div class="tree-box">
    <a-directory-tree
      v-model:expandedKeys="expandedKeys"
      v-model:selectedKeys="selectedKeys"
      :tree-data="treeData"
      :selectable="false"
      :fieldNames="{ children: 'children', title: 'name', key: 'key' }"
    >
      <template #title="{ key: treeKey, title, path, is_dir }">
        <span class="title">
          <sapn class="title-left">{{ title }}</sapn>
          <span>
            <a-popover title="">
              <template #content>
                <div v-if="is_dir">
                  <p
                    @click="renameDir(path, title, treeKey)"
                    style="cursor: pointer"
                  >
                    重新命名
                  </p>
                  <p
                    @click="deleteDir(path, title, treeKey)"
                    style="cursor: pointer"
                  >
                    删除分类
                  </p>
                </div>
                <div v-else>
                  <p
                    @click="renameDir(path, title, treeKey)"
                    style="cursor: pointer"
                  >
                    删除笔记
                  </p>
                </div>
              </template>
              <img
                style="width: 15px; height: 15px"
                src="./assets/more.png"
                alt=""
              /> </a-popover
          ></span>
          <span  v-if="is_dir">
            <a-tooltip>
              <template #title>新建笔记</template>
              <img
                style="width: 15px; height: 15px; margin-right: 10px"
                src="./assets/add.png"
                alt=""
                @click="addNote(path, title, treeKey)"
              />
            </a-tooltip>
          </span>
        </span>
      </template>
    </a-directory-tree>
  </div>
</template>
<script lang="ts" setup>
import type { TreeProps } from "ant-design-vue";
import { useStore } from "../../store/index";
import { invoke } from "@tauri-apps/api";
import { ref, reactive } from "vue";
const Store = useStore();
console.log(Store);
const expandedKeys = ref<string[]>(["0-0", "0-1"]);
const selectedKeys = ref<string[]>([]);
const treeData: TreeProps["treeData"] = reactive([]);
const lookDataDir = async () => {
  treeData.length = 0;
  let data: any = await invoke("get_directory_contents", { path: "data" });
  console.log(data);
  treeData.push(...data);
  console.log(treeData);
};
lookDataDir();
// 文件夹重命令
const renameDir = (path: string, title: string, treeKey: string) => {
  console.log(path, title, treeKey);
};
// 删除文件夹
const deleteDir = (path: string, title: string, treeKey: string) => {
  console.log(path, title, treeKey);
};
//添加笔记
const addNote = async (path: string, title: string, treeKey: string) => {
  console.log(path, title, treeKey);
  Store.addNote(`新建笔记${Date.now()}`);
  console.log(Store.noteName, "notename");
  console.log("data\\" + title + "\\" + Store.noteName + ".md");
  try {
    let ok = await invoke("create_file", {
      path: "data/" + title + "/" + Store.noteName + ".md",
    });
    console.log(ok);
    // lookDataDir();
  } catch (error) {
    console.log("create_file", error);
  }
};
</script>

<style lang="less" scoped>
.tree-box {
  width: 230px;
  border-right: 1px solid #ccc;
  :deep(.ant-tree-node-content-wrapper) {
    display: flex;
    width: 100%;
    .title {
      width: auto;
      display: flex;
      gap: 10px;
      align-items: center;
      .title-left {
        width: 100px;
        overflow: hidden;
        text-wrap: nowrap;
      }
    }
  }
}
</style>
