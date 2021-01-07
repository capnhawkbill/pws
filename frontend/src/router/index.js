import { createRouter, createWebHashHistory } from 'vue-router'
import Home from '../views/Home.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/rang',
    name: 'Rang',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/Rang.vue')
  },
  {
    path: '/klas',
    name: 'Klas',
    component: () => import('../views/Klas.vue')
  },
  {
    path: '/winkel',
    name: 'Winkel',
    component: () => import('../views/Winkel.vue')
  },
  {
    path: '/profiel',
    name: 'Profiel',
    component: () => import('../views/Profiel.vue')
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('../views/Login.vue')
  },
  {
    path: '/aanmelden',
    name: 'Aanmelden',
    component: () => import('../views/Aanmelden.vue')
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
