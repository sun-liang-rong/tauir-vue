import { createRouter, createWebHistory } from 'vue-router';
import Layout from '../layout/index.vue';
import Login from '../pages/login/index.vue'
const routes = [
  {
    path: '/',
    component: Login
  },
  {
    path: '/login',
    component: Layout
  }
]
const router = createRouter({
  routes: routes,
  history: createWebHistory()
})
export default router;