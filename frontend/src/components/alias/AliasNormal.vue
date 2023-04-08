<script setup lang='ts'>
import { useAlias } from '../../store/alias'
import type { Alias } from '../../types/Alias'

const props = defineProps<{ data: Alias }>()
const alias = useAlias()
</script>

<template>
  <button
    :key="props.data.name"
    class="alias-item"
    :class="{ 'alias-active': props.data.name === alias.active }"
    @click="alias.$patch({ active: props.data.name })"
  >
    <div class="thumbnail">
      <img
        v-if="props.data.type !== 'text'"
        :src="props.data.content"
        :alt="`!${props.data.name}`"
      >
      <p v-else :title="props.data.content">
        {{ props.data.content }}
      </p>
    </div>

    <div class="name">
      <span>{{ props.data.name }}</span>
    </div>
  </button>
</template>
