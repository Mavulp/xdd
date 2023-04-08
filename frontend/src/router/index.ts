import { createRouter, createWebHistory } from 'vue-router'

import { useUser } from '../store/user'
import afterEach from './guards/afterEach'
import beforeResolve from './guards/beforeResolve'

import RouteHome from './views/RouteHome.vue'
import RouteCreate from './views/RouteCreate.vue'
import RouteAuthorize from './views/RouteAuthorize.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      // Special route used as a catch-all.
      path: '/:pathMatch(.*)*',
      // In case user wants to access a path which is not defined, they will be
      // redirected to the following route
      redirect: {
        name: 'RouteHome',
      },
    },
    {
      // The path to the the page
      path: '/home',
      // Name is used for matching routes. Allowing you to change url to the
      // page without having to update all the
      name: 'RouteHome',
      component: RouteHome,
      // The meta object contains whatever properties you want In our
      meta: {
        title: 'Alias List',
        requiresAuth: true,
      },
    },
    {
      path: '/create',
      name: 'RouteCreate',
      component: RouteCreate,
      meta: {
        title: 'New Alias',
        requiresAuth: true,
      },
      // Disable entry to the route if user does not have the necessary
      // permissions
      beforeEnter() {
        const user = useUser()
        return user.can('create-aliases')
      },
    },
    {
      path: '/authorize',
      name: 'RouteAuthorize',
      component: RouteAuthorize,
      meta: {
        title: 'You are being authorized. Wait! NO DONT CLICK ANYTHING!!',
      },
    },
  ],
})

// Register router guards
router.afterEach(afterEach)
router.beforeResolve(beforeResolve)

export default router
