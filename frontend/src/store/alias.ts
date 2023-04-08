import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import type { Alias } from '../types/Alias'
import type { PostAlias } from '../types/PostAlias'
import { LOAD } from '../js/definitions'
import { del, get, post } from '../js/fetch'
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

    post('/alias', form)
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

  function remove(name: Alias['name']) {
    const { add, del: _del } = useLoading()
    const { push } = useToast()
    add(LOAD.DELETE)
    del(`/alias/${name}`)
      .then(() => {
        push({
          type: 'success',
          message: `Successfully removed alias "${name}"`,
        })

        list.value = list.value.filter(l => l.name !== name)
      })
      .catch(({ message }) => push({
        type: 'error',
        message,
      }))
      .finally(() => _del(LOAD.DELETE))
  }

  async function fetch() {
    const { add, del } = useLoading()
    add(LOAD.FETCH)
    list.value = await get<Alias[]>('/alias').catch(() => [])
    del(LOAD.FETCH)
  }

  return {
    list,
    categories,
    add,
    remove,
    fetch,
    active,
    activeAlias,
  }
})
