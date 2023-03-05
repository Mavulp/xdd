/**
 * For global state we use pinia. It allows us to access a singele point of truth for data
 * anywhere else in the app
 * 
 * I recommend checking out https://pinia.vuejs.org/core-concepts/
 */

import { defineStore } from 'pinia'

interface State {
  loading: Set<string>
}

/**
 * This simple store can be used to temporarily store an arbitrary ID.
 * In other parts of the app we can check if this ID is currently being stored here
 * and alter the UI.
 * 
 * Example:
 * 
 * ```
 * loading.add('users')
 * await fetch('/api/users)
 * loading.del('users')
 * ```
 * 
 * In the time the async function ran, the UI can display a loading bar or a spinner.
 * Anywhere in the app we can then use this
 * 
 * ```
 * const loading = useLoading()
 * 
 * // And in the UI we write
 * <Spinner v-if="loading.get('users')" 
 * ```
 */

export const useLoading = defineStore('loading', {
  state: () => ({
    // We use set instead of an array as it stores only unique values. 
    loading: new Set(),
  } as State),
  actions: {
    // Add item(s) to loading
    add(...items: Array<string>) {
      if (items.length > 0) {
        for (const item of items)
          this.loading.add(item)
      }
    },
    // Remove item(s) from loading
    del(...items: Array<string>) {
      if (items.length > 0) {
        for (const item of items)
          this.loading.delete(item)
      }
    },
  },
  getters: {
    // Check wether an item is currently loading
    get:
      state =>
        (...items: Array<string>) => {
          if (items.length > 0)
            return Array.from(state.loading).some((item: any) => items.includes(item))

          return state.loading.size > 0
        },
  },
})
