import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import type { Alias } from '../types/Alias'
import type { PostAlias } from '../types/PostAlias'
import type { PutAlias } from '../types/PutAlias'
import { LOAD } from '../js/definitions'
import { del, get, post, put } from '../js/fetch'
import type { AliasType } from '../types/AliasType'
import { useLoading } from './loading'
import { useToast } from './toast'

export const categoryLabels: Record<AliasType, string> = {
  animatedEmote: 'Animated Emote',
  emote: 'Emote',
  gif: 'Gif',
  image: 'Image',
  text: 'Text',
}

export const useAlias = defineStore('alias', () => {
  const list = ref<Alias[]>([])
  const categories = computed(() => list.value.reduce((group, item) => {
    const type = item.type
    if (group.includes(type))
      return group
    group.push(type)
    return group
  }, [] as AliasType[]))

  // Active alias showing up in modal
  const active = ref<string>()
  const activeAlias = computed(() => list.value.find(a => a.name === active.value))

  async function add(form: PostAlias) {
    const { add, del } = useLoading()
    const { push } = useToast()
    add(LOAD.CREATE)

    return post('/alias', form)
      .then((newAlias) => {
        push({
          type: 'success',
          message: `Successfully added alias "${form.name}"`,
        })

        list.value.push(newAlias)
      })
      .catch(({ message }) => push({
        type: 'error',
        message,
      }))
      .finally(() => del(LOAD.CREATE))
  }

  async function remove(name: Alias['name']) {
    const { add, del: _del } = useLoading()
    const { push } = useToast()
    add(LOAD.DELETE)
    return del(`/alias/${name}`)
      .then(() => {
        push({
          type: 'success',
          message: `Successfully removed "${name}"`,
        })

        list.value = list.value.filter(l => l.name !== name)
      })
      .catch(({ message }) => push({
        type: 'error',
        message,
      }))
      .finally(() => _del(LOAD.DELETE))
  }

  const cacheTimeout = ref<number>()
  const cacheLimitInSeconds = 3600
  async function fetch() {
    if (cacheTimeout.value && Date.now() - cacheTimeout.value < (cacheLimitInSeconds * 1000))
      return Promise.resolve(list.value)

    const { add, del } = useLoading()
    add(LOAD.FETCH)
    return get<Alias[]>('/alias')
      .then((res) => {
        list.value = res
        return res
      })
      .catch(() => [])
      .finally(() => {
        del(LOAD.FETCH)
        cacheTimeout.value = Date.now()
      })
  }

  async function edit(name: Alias['name'], form: PutAlias) {
    const { add, del: _del } = useLoading()
    const { push } = useToast()
    add(LOAD.EDIT)
    return put(`/alias/${name}`, form)
      .then(() => {
        push({
          type: 'success',
          message: `Successfully updated "${name}"`,
        })

        const index = list.value.findIndex(a => a.name === name)
        const original = { ...list.value[index] }
        Object.assign(original, form)
        list.value.splice(index, 1, original)
      })
      .catch(({ message }) => push({
        type: 'error',
        message,
      }))
      .finally(() => _del(LOAD.EDIT))
  }

  return {
    list,
    categories,
    add,
    edit,
    remove,
    fetch,
    active,
    activeAlias,
  }
})
