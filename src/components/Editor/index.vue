<template>
  <div class="container">
    <quill-editor
      class="ql-editor"
      v-model:content="content"
      ref="myQuillEditor"
      contentType="html"
      :options="editorOption"
      @blur="onEditorBlur($event)"
      @focus="onEditorFocus($event)"
      @change="onEditorChange($event)"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
const content = ref('sssss');
  const editorOption = {
    modules: {
      toolbar: [
        ['bold', 'italic', 'underline', 'strike'], // 加粗 斜体 下划线 删除线
        ['blockquote', 'code-block'], // 引用  代码块
        [{ header: 1 }, { header: 2 }], // 1、2 级标题
        [{ list: 'ordered' }, { list: 'bullet' }], // 有序、无序列表
        [{ script: 'sub' }, { script: 'super' }], // 上标/下标
        [{ indent: '-1' }, { indent: '+1' }], // 缩进
        [{ direction: 'rtl' }], // 文本方向
        [{ size: ['12px', false, '16px', '18px', '20px', '30px'] }], // 字体大小
        [{ header: [1, 2, 3, 4, 5, 6, false] }], // 标题
        [{ color: [] }, { background: [] }], // 字体颜色、字体背景颜色
        [{ font: [false, 'SimSun', 'SimHei', 'Microsoft-YaHei', 'KaiTi', 'FangSong', 'Arial'] }], // 字体种类
        [{ align: [] }], // 对齐方式
        ['clean'], // 清除文本格式
        ['link', 'image', 'video'], // 链接、图片、视频
      ],
    },
    placeholder: '请输入正文',
  };
  // 失去焦点事件
  const onEditorBlur = (quill: any) => {
    console.log('editor blur!', quill);
    console.log(content.value, 'content')
  };
  // 获得焦点事件
  const onEditorFocus = (quill: any) => {
    console.log('editor focus!', quill);
  };
  // 准备富文本编辑器
  // const onEditorReady = (quill: any) => {
  //   console.log('editor ready!', quill);
  // };
  // 内容改变事件
  const onEditorChange = ({ quill, html, text }: any) => {
    console.log('editor change!', quill, html, text);
    content.value = html;
  };
  onMounted(() => {
    let qlToolbar = document.getElementsByClassName("ql-toolbar")[0] as HTMLElement;
    console.log(qlToolbar, 'qlToolbar')
    console.log(qlToolbar.clientHeight)
    let qlContainer = document.getElementsByClassName("ql-container")[0] as HTMLElement;
    if(qlContainer) {
      qlContainer.style.height = `calc(100% - ${qlToolbar.clientHeight}px)`
    }
  })
</script>

<style lang="less" scoped>
.container {
  width: 100%;
  height: 100%;
  .ql-editor {
    width: 100%;
    height: 100%;
  }
}
</style>
