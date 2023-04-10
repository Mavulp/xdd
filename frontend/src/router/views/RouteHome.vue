<script setup lang='ts'>
import { computed, onBeforeMount, onMounted, ref, watch } from 'vue'
import { useOffsetPagination, useWindowScroll } from '@vueuse/core'
import { useRouteHash } from '@vueuse/router'
import { categoryLabels, useAlias } from '../../store/alias'
import { useLoading } from '../../store/loading'
import { LOAD } from '../../js/definitions'
import { searchInStr } from '../../js/utils'

import Spinner from '../../components/generic/Spinner.vue'
import AliasNormal from '../../components/alias/AliasNormal.vue'
import AliasInline from '../../components/alias/AliasInline.vue'
import AliasModal from '../../components/alias/AliasModal.vue'

/**
 * This is the wrapper component around each 'page'.
 *
 * In most cases you just create the layout and import components for each section here.
 * Not much logic is usually done here, except fetching data and/or displaying loading states etc
 */

const loading = useLoading()
const alias = useAlias()

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
  alias.fetch()
})

const filteredAliases = computed(() => {
  let data = alias.list
  if (search.value)
    data = data.filter(item => searchInStr([item.author, item.name], search.value))

  if (filter.value.size > 0)
    data = data.filter(item => filter.value.has(item.type))

  return data
})

// Pagination setup
const paginationSize = 50
const {
  currentPage,
  prev,
  next,
  isFirstPage,
  isLastPage,
} = useOffsetPagination({
  total: filteredAliases.value.length,
  pageSize: paginationSize,
  page: 1,
})

// Pagination array to render
const aliasesToRender = computed(() => {
  const start = (currentPage.value - 1) * paginationSize
  const end = start + paginationSize - 1
  return filteredAliases.value.slice(start, end)
})

// Display button up if user has scrolled
const { y } = useWindowScroll()

function goUp() {
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

// Update url
// watch(() => alias.active, )
const query = useRouteHash()
watch(() => alias.active, (value) => {
  query.value = value ? `#${value}` : ''
})

onMounted(() => {
  if (query.value) {
    alias.$patch({
      active: query.value.substring(1),
    })
  }
  else if (alias.active) {
    query.value = `#${alias.active}`
  }
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
          <input
            v-model="search"
            type="text"
            :placeholder="`Search through ${alias.list.length} aliases...`"
          >
        </div>

        <div class="list-controls">
          <div class="list-types">
            <button
              v-for="item in alias.categories"
              :key="item"
              class="button"
              :class="[filter.has(item) ? 'btn-accent' : 'btn-white']"
              @click="toggleItem(item)"
            >
              {{ categoryLabels[item] }}
            </button>
          </div>

          <div class="flex-1" />

          <div v-if="filteredAliases.length > paginationSize" class="list-pagination">
            <button class="button btn-white btn-icon" :disabled="isFirstPage" @click="prev">
              <Icon icon="mdi:chevron-left" />
            </button>
            <span>
              {{ currentPage }}
            </span>
            <button class="button btn-white btn-icon" :disabled="isLastPage" @click="next">
              <Icon icon="mdi:chevron-right" />
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

      <div v-if="loading.get(LOAD.FETCH)" class="list-loading">
        <Spinner />
      </div>

      <template v-else-if="alias.list.length === 0">
        <div class="list-empty">
          <p>There are no aliases right now.</p>
          <router-link :to="{ name: 'RouteCreate' }" class="button btn-gray">
            Add Alias
          </router-link>
        </div>
      </template>

      <div
        v-else
        class="list"
        :class="{
          'large-icons': displayType === 'large-icon',
          'is-inline': displayType === 'list',
        }"
      >
        <template v-if="displayType !== 'list'">
          <AliasNormal v-for="item in aliasesToRender" :key="item.name" :data="item" />
        </template>
        <template v-else>
          <AliasInline v-for="item in aliasesToRender" :key="item.name" :data="item" />
        </template>
      </div>

      <button :style="{ opacity: y > 250 ? 1 : 0 }" class="button btn-white btn-icon btn-go-up" @click="goUp()">
        <Icon icon="mdi:arrow-up" />
      </button>
    </div>

    <footer>
      <p class="made-by">
        <Icon icon="mdi:code" />
        Made by <a href="https://github.com/mavulp" target="_blank">Mavulp</a> in 2023
      </p>
    </footer>

    <AliasModal />
  </div>
</template>
