import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    component: () => import('../views/home.vue')
  },
  {
    path: '/leerling',
    component: () => import('@/components/leerlingmenu.vue'),
    children: [
      {
        path: '',
        name: 'leerling.home',
        component: () => import('../views/leerling/home.vue'),
      },
      {
        path: 'rang',
        name: 'leerling.rang',
        component: () => import('../views/leerling/rang.vue'),
      },
      {
        path: 'klas',
        name: 'leerling.klas',
        component: () => import('../views/leerling/klas.vue')
      },
      {
        path: 'winkel',
        name: 'leerling.winkel',
        component: () => import('../views/leerling/winkel.vue')
      },
      {
        path: 'profiel',
        name: 'leerling.profiel',
        component: () => import('../views/leerling/profiel.vue')
      },
      {
        path: 'login',
        name: 'leerling.login',
        component: () => import('../views/leerling/login.vue')
      },
      {
        path: 'aanmelden',
        name: 'leerling.aanmelden',
        component: () => import('../views/leerling/aanmelden.vue')
      }
    ],
  },
  {
    path: '/leraar',
    component: () => import('@/components/leraarmenu.vue'),
    children: [
      {
        path: '',
        name: 'leraar.home',
        component: () => import('../views/leraar/home.vue'),
      },
      {
        path: 'klassen',
        name: 'leraar.klassen',
        component: () => import('../views/leraar/klassen.vue')
      },
      {
        path: 'profiel',
        name: 'leraar.profiel',
        component: () => import('../views/leraar/profiel.vue')
      },
      {
        path: 'login',
        name: 'leraar.login',
        component: () => import('../views/leraar/login.vue')
      },
      {
        path: 'aanmelden',
        name: 'leraar.aanmelden',
        component: () => import('../views/leraar/aanmelden.vue')
      }
    ],
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
