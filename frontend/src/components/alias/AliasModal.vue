<script setup lang='ts'>
// This is the pop-up component which displays information about an alias and is
// visible as long as a component is clicked on

import { useDateFormat } from '@vueuse/shared'
import { computed } from 'vue'
import { categoryLabels, useAlias } from '../../store/alias'

const alias = useAlias()
const active = computed(() => alias.activeAlias)

const date = useDateFormat(Number(active.value?.createdAt ? Number(active.value.createdAt) * 1000 : 0), 'D MMMM YYYY')
</script>

<template>
  <div class="alias-modal" :class="{ 'modal-active': alias.activeAlias }">
    <button class="button btn-icon btn-white btn-close" @click="alias.$patch({ active: undefined })">
      <Icon icon="mdi:close-thick" />
    </button>
    <div v-if="active" class="modal-wrapper">
      <div class="content">
        <div v-if="active.type !== 'text'" class="thumbnail">
          <img :src="active.content" :alt="`!${active.name}`">
        </div>
        <div v-else class="text">
          <p>{{ active.content }}</p>
        </div>
      </div>
      <div class="info">
        <span class="category">{{ categoryLabels[active.type] }}</span>
        <button class="name">
          {{ active.name }}

          <div class="flex-1" />

          <Icon icon="mdi:content-copy" />
        </button>

        <p class="meta">
          Added {{ date }} by {{ active.author }}
        </p>
      </div>
    </div>
  </div>
</template>
