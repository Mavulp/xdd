<script setup lang='ts'>
import { onBeforeMount, ref } from 'vue'
import { get } from '../../js/fetch'
import type { Alias } from '../../types/Alias'

/**
 * This is the wrapper component around each 'page'.
 *
 * In most cases you just create the layout and import components for each section here.
 * Not much logic is usually done here, except fetching data and/or displaying loading states etc
 */

type DisplayTypes = 'small-icon' | 'large-icon' | 'list'

const search = ref('')
const displayType = ref<DisplayTypes>('small-icon')
const filter = ref(new Set())

const types = ['Text', 'Emotes', 'Photos', 'Gifs']

function toggleItem(item: string) {
  if (filter.value.has(item))
    filter.value.delete(item)
  else
    filter.value.add(item)
}

/**
 * List
 */

const data = ref<Alias[]>([])

onBeforeMount(async () => {
  data.value = await get<Alias[]>('/api/alias').catch(() => ([]))
})

/**
 * This is testing data for list
 */
</script>

<template>
  <div class="route-home">
    <div class="container mid">
      <div class="list-search">
        <div class="input-wrap" :class="{ 'has-input': search }">
          <div class="icon">
            <Icon icon="ph:magnifying-glass-bold" />
          </div>
          <input v-model="search" type="text" placeholder="Search for alias">
        </div>

        <div class="list-controls">
          <div class="list-types">
            <button
              v-for="item in types"
              :key="item"
              class="button"
              :class="[filter.has(item) ? 'btn-accent' : 'btn-white']"
              @click="toggleItem(item)"
            >
              {{ item }}
            </button>
          </div>

          <div class="list-display-types">
            <button
              class="button btn-icon"
              data-title-bottom="Small Icons"
              :class="[displayType === 'small-icon' ? 'btn-accent' : 'btn-white']"
              @click="displayType = 'small-icon'"
            >
              <Icon icon="ph:dots-nine-bold" />
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
              <Icon icon="ph:list-bullets" />
            </button>
          </div>
        </div>
      </div>

      <template v-if="data.length === 0">
        <div class="list-empty">
          <p>There are no aliases right now.</p>
          <button class="button btn-gray">
            Add Alias
          </button>
        </div>
      </template>

      <div v-else class="list">
        <button
          v-for="item in data"
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
