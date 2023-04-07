<script setup lang='ts'>
import { computed, onBeforeMount, ref } from 'vue'
import { categoryLabels, useEmotes } from '../../store/emotes'
import Spinner from '../../components/generic/Spinner.vue'
import { useLoading } from '../../store/loading'
import { LOAD } from '../../js/definitions'
import { searchInStr } from '../../js/utils'

/**
 * This is the wrapper component around each 'page'.
 *
 * In most cases you just create the layout and import components for each section here.
 * Not much logic is usually done here, except fetching data and/or displaying loading states etc
 */

const loading = useLoading()
const emotes = useEmotes()

type DisplayTypes = 'small-icon' | 'large-icon' | 'list'

const search = ref('')
const displayType = ref<DisplayTypes>('small-icon')
const filter = ref(new Set())

function toggleItem(item: string) {
  if (filter.value.has(item))
    filter.value.delete(item)
  else
    filter.value.add(item)
}

onBeforeMount(async () => {
  emotes.fetch()
})

const filteredAliases = computed(() => {
  let data = emotes.aliases
  if (search.value)
    data = data.filter(item => searchInStr([item.author, item.name], search.value))

  if (filter.value.size > 0)
    data = data.filter(item => filter.value.has(item.type))

  return data
})
</script>

<template>
  <div class="route-home">
    <div class="container mid">
      <div class="list-search">
        <div class="input-wrap" :class="{ 'has-input': search }">
          <div class="icon">
            <Icon icon="mdi:magnify" />
          </div>
          <input v-model="search" type="text" placeholder="Search for alias">
        </div>

        <div class="list-controls">
          <div class="list-types">
            <button
              v-for="item in emotes.categories"
              :key="item"
              class="button"
              :class="[filter.has(item) ? 'btn-accent' : 'btn-white']"
              @click="toggleItem(item)"
            >
              {{ categoryLabels[item] }}
            </button>
          </div>

          <div class="list-display-types">
            <button
              class="button btn-icon"
              data-title-bottom="Small Icons"
              :class="[displayType === 'small-icon' ? 'btn-accent' : 'btn-white']"
              @click="displayType = 'small-icon'"
            >
              <Icon icon="mdi:dots-grid" />
            </button>
            <button
              class="button btn-icon"
              data-title-bottom="Large Icons"
              :class="[displayType === 'large-icon' ? 'btn-accent' : 'btn-white']"
              @click="displayType = 'large-icon'"
            >
              <Icon icon="ph:squares-four-fill" />
            </button>
            <button
              class="button btn-icon"
              data-title-bottom="List"
              :class="[displayType === 'list' ? 'btn-accent' : 'btn-white']"
              @click="displayType = 'list'"
            >
              <Icon icon="mdi:format-list-bulleted-square" />
            </button>
          </div>
        </div>
      </div>

      <div v-if="loading.get(LOAD.FETCH)">
        <Spinner />
      </div>

      <template v-else-if="emotes.aliases.length === 0">
        <div class="list-empty">
          <p>There are no aliases right now.</p>
          <router-link :to="{ name: 'RouteCreate' }" class="button btn-gray">
            Add Alias
          </router-link>
        </div>
      </template>

      <div v-else class="list">
        <button
          v-for="item in filteredAliases"
          :key="item.name"
          class="list-item"
        >
          <div class="thumbnail">
            <img :src="item.content" alt="">
          </div>

          <div class="name">
            <span>{{ item.name }}</span>
          </div>
        </button>
      </div>
    </div>
  </div>
</template>
