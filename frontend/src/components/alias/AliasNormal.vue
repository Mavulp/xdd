<script setup lang='ts'>
import type { Alias } from '../../types/Alias'

const props = defineProps<{ data: Alias }>()
</script>

<template>
  <button
    :key="props.data.name"
    class="alias-item"
  >
    <div class="thumbnail">
      <img
        v-if="props.data.type !== 'text'"
        :src="props.data.content"
        :alt="props.data.name"
      >
      <p v-else>
        {{ props.data.content }}
      </p>
    </div>

    <div class="name">
      <span>{{ props.data.name }}</span>
    </div>
  </button>
</template>
