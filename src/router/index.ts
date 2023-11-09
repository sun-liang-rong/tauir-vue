import { createRouter, createWebHistory } from 'vue-router';
import Layout from '../layout/index.vue';
import Home from '../pages/home/index.vue'
const routes = [
  {
    path: '/',
    component: Layout,
    children: [
      {
        path: '',
        component: Home
      }
    ]
  }
]
const router = createRouter({
  routes: routes,
  history: createWebHistory()
})
export default router;