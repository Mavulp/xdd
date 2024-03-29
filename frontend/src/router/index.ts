import { createRouter, createWebHistory } from 'vue-router'

import { useUser } from '../store/user'
import afterEach from './guards/afterEach'
import beforeResolve from './guards/beforeResolve'

import RouteHome from './views/RouteHome.vue'
import RouteCreate from './views/RouteCreate.vue'
import RouteAuthorize from './views/RouteAuthorize.vue'
import RouteSignOutVue from './views/RouteSignOut.vue'

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
      path: '/aliases',
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
        // If user is not allowed, re-route back to list
        if (!user.can('create-aliases'))
          return { name: 'RouteHome' }
        return true
      },
    },
    {
      path: '/edit/:name',
      name: 'RouteEdit',
      component: RouteCreate,
      meta: {
        title: 'Edit Alias',
        requiresAuth: true,
      },
      beforeEnter() {
        const user = useUser()
        // If user is not allowed, re-route back to list
        if (!user.can('edit-aliases'))
          return { name: 'RouteHome' }
        return true
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
    {
      path: '/sign-out',
      name: 'RouteSignOut',
      component: RouteSignOutVue,
      meta: {
        title: 'You just logged out. Wanna come back?',
      },
      // Allow users to only enter this page if they are logged out
      beforeEnter() {
        const user = useUser()
        if (user.isSignedIn)
          return { name: 'RouteHome' }
        return !user.isSignedIn
      },
    },
  ],
})

// Register router guards
router.afterEach(afterEach)
router.beforeResolve(beforeResolve)

export default router
