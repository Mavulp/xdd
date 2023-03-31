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

function toggleItem(item) {
  if (filter.value.has(item))
    filter.value.delete(item)
  else
    filter.value.add(item)
}
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
            :class="{ 'btn-accent': filter.has(item) }"
            @click="toggleItem(item)"
          >
            {{ item }}
          </button>
          <!-- <button class="button">
            Emotes
          </button>
          <button class="button">
            Photos
          </button>
          <button class="button">
            Gifs
          </button> -->
        </div>

        <div class="list-display-types">
          <button
            class="button btn-icon"
            data-title-bottom="Small Icons"
            :class="{ 'btn-accent': displayType === 'small-icon' }"
            @click="displayType = 'small-icon'"
          >
            <Icon icon="ph:dots-nine-bold" />
          </button>
          <button
            class="button btn-icon"
            data-title-bottom="Large Icons"
            :class="{ 'btn-accent': displayType === 'large-icon' }"
            @click="displayType = 'large-icon'"
          >
            <Icon icon="ph:squares-four-fill" />
          </button>
          <button
            class="button btn-icon"
            data-title-bottom="List"
            :class="{ 'btn-accent': displayType === 'list' }"
            @click="displayType = 'list'"
          >
            <Icon icon="ph:list-bullets" />
          </button>
        </div>
      </div>
    </div>
    Hello World
  </div>
</template>
