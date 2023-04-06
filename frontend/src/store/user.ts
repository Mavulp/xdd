import { isNil } from 'lodash-es'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { TOKEN_KEY } from '../js/config'

export const useUser = defineStore('user', () => {
  const username = ref('')
  const isSignedIn = ref(!isNil(localStorage.getItem(TOKEN_KEY)))
  const permissions = ref<string[]>([])

  return {
    username,
    isSignedIn,
    permissions,
  }
})
