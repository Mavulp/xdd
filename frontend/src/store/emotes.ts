import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import type { Alias } from '../types/Alias'
import type { PostAlias } from '../types/PostAlias'
import { LOAD } from '../js/definitions'
import { get, post } from '../js/fetch'
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

export const useEmotes = defineStore('emotes', () => {
  const aliases = ref<Alias[]>([])
  const categories = computed(() => aliases.value.reduce((group, item) => {
    const type = item.type
    if (group.includes(type))
      return group
    group.push(type)
    return group
  }, [] as AliasType[]))
  const active = ref<string>()

  async function addAlias(form: PostAlias) {
    const { add, del } = useLoading()
    const { push } = useToast()
    add(LOAD.CREATE)

    post('/alias', form)
      .then((newAlias) => {
        push({
          type: 'success',
          message: `Successfully added alias "${form.name}""`,
        })

        aliases.value.push(newAlias)
      })
      .catch(({ message }) => push({
        type: 'error',
        message,
      }))
      .finally(() => del(LOAD.CREATE))
  }

  async function fetch() {
    const { add, del } = useLoading()
    add(LOAD.FETCH)
    aliases.value = await get<Alias[]>('/alias').catch(() => [])
    del(LOAD.FETCH)
  }

  return {
    aliases,
    categories,
    addAlias,
    fetch,
    active,
  }
})
