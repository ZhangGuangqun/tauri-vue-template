import Vue from 'vue'
import VueRouter from 'vue-router'

Vue.use(VueRouter)

import Layout from '@/layout/index.vue'

import home from '@/pages/home'

const routes = [
  {
    path: '/',
    component: Layout,
    children: [
        { path: '', component: home },
    ]
  },
]

const router = new VueRouter({
  routes
})

export default router
