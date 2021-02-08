import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'start',
    component: () => import('@/views/start.vue')
  },
  {
    path: '/:user/login',
    name: 'login',
    component: () => import('@/views/login.vue')
  },
  {
    path: '/:user/aanmelden',
    name: 'aanmelden',
    component: () => import('@/views/aanmelden.vue')
  },
  {
    path: '/:user',
    component: () => import('@/components/menu.vue'),
    children: [
      {
        path: 'home',
        name: 'home',
        component: () => import('@/views/home.vue'),
      },
      {
        path: 'klassen',
        name: 'klassen',
        component: () => import('@/views/klassen.vue')
      },
      {
        path: 'profiel',
        name: 'profiel',
        component: () => import('@/views/profiel.vue')
      },
      {
        path: 'klassen/:id',
        name: 'klas',
        component: () => import('@/views/klas.vue')
      },
      {
        path: 'klassen/join/:id',
        name: 'join',
        component: () => import('@/views/joinklas.vue')
      },
      {
        path: 'klassen/aanmaken',
        name: 'klas.aanmaken',
        component: () => import('@/views/aanmaken.vue')
      },
      {
        path: 'klassen/:id/:hw',
        name: 'huiswerk',
        component: () => import('@/views/huiswerk.vue')
      },
      {
        path: 'klassen/:id/huiswerk',
        name: 'huiswerk.aanmaken',
        component: () => import('@/views/addhuiswerk.vue')
      },
    ],
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
