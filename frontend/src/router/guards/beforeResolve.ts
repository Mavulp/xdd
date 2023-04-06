import { isEmpty } from 'lodash-es'
import type { NavigationGuardNext, RouteLocationNormalized } from 'vue-router'
import { get } from '../../js/fetch'
import { parseJwt } from '../../js/utils'
import { useUser } from '../../store/user'
import { API_URL, TOKEN_KEY, __LOCALHOST__ } from '../../js/config'
import { useLoading } from '../../store/loading'
import { LOAD } from '../../js/definitions'

function setupUser(token: string) {
  const user = useUser()
  const { name, groups } = parseJwt(token)

  user.$patch({
    username: name,
    permissions: groups,
    isSignedIn: true,
  })
}

export default async function (to: RouteLocationNormalized, from: RouteLocationNormalized, next: NavigationGuardNext) {
  const user = useUser()
  const token = localStorage.getItem(TOKEN_KEY)

  if (token && !user.username)
    setupUser(token)

  if (to.meta.requiresAuth || to.name === 'RouteAuthorize') {
    if (!token) {
      const loading = useLoading()
      const { token, redirect_uri } = to.query as { token: string; redirect_uri: string }

      if (isEmpty(token)) {
        localStorage.removeItem(TOKEN_KEY)

        if (__LOCALHOST__) {
          window.location.replace('https://account.hivecom.net/login?service=xdd-dev&redirect_to=%2F')
        }
        else {
          loading.add(LOAD.APP)

          fetch(`${API_URL}/account/login`, {
            method: 'GET',
            redirect: 'follow',
            credentials: 'same-origin',
          })
            .then((res: any) => {
              if (res.redirected) {
                const redirectedToUrl = res.url
                window.location.replace(redirectedToUrl)
              }
            })
        }
      }
      else {
        loading.add(LOAD.APP)

        localStorage.setItem(TOKEN_KEY, token)
        setupUser(token)

        await get(`/auth/authorize?redirect_uri=${redirect_uri}&token=${token}`)
          .finally(() => {
            loading.del(LOAD.APP)
            return next({ name: 'RouteHome' })
          })
      }
    }

    next()
  }
}
