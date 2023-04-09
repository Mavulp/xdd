import { isNil } from 'lodash-es'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { TOKEN_KEY } from '../js/config'

export const useUser = defineStore('user', () => {
  const username = ref('')
  const isSignedIn = ref(!isNil(localStorage.getItem(TOKEN_KEY)))
  const permissions = ref<string[]>([])

  function reset() {
    username.value = ''
    isSignedIn.value = false
    permissions.value = []
  }

  // Check wether user object includes the provided permission(s)
  function can(allowed: string[] | string) {
    if (typeof allowed === 'string')
      allowed = [allowed]
    return allowed.some(item => permissions.value.includes(item))
  }

  return {
    can,
    reset,
    username,
    isSignedIn,
    permissions,
  }
})
