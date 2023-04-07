<script setup lang='ts'>
import type { Alias } from '../../types/Alias'

const props = defineProps<{ data: Alias }>()
</script>

<template>
  <button
    :key="props.data.name"
    class="alias-item"
  >
    <pre>
      {{ props.data }}
    </pre>
  </button>
</template>
