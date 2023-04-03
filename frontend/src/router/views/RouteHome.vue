<script setup lang='ts'>
import { ref } from 'vue'

/**
 * This is the wrapper component around each 'page'.
 *
 * In most cases you just create the layout and import components for each section here.
 * Not much logic is usually done here, except fetching data and/or displaying loading states etc
 */

// TODO: Split into components
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
 * This is testing data for list
 */
const test = ref<Array<{
  name: string
  content: string
}>>([])
const loading = ref(false)

async function createTestData() {
  loading.value = true
  const data = []

  for (let i = 0; i <= 100; i++) {
    data.push({
      name: `${i + 1}`,
      content: 'https://picsum.photos/200/200',
    })
  }

  test.value = data
}

createTestData()
</script>

<template>
  <div class="route-home">
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

    <div class="list">
      <button
        v-for="item in test"
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
</template>
