import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import router from './router/index';
import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css';
import { QuillEditor } from '@vueup/vue-quill';
import '@vueup/vue-quill/dist/vue-quill.snow.css';
import { createPinia } from 'pinia'
const app = createApp(App)
app.component('QuillEditor', QuillEditor);
app.use(createPinia()).use(router).use(Antd).mount("#app");
