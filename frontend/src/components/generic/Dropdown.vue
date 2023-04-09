<script setup lang='ts'>
import { flip, offset, useFloating } from '@floating-ui/vue'
import { computed, ref } from 'vue'
import { onClickOutside } from '@vueuse/core'

const open = ref(false)
const anchor = ref()
const dropdown = ref()

onClickOutside(anchor, () => {
  open.value = false
})

// FLoating
const { x, y, strategy } = useFloating(anchor, dropdown, {
  placement: 'bottom-end',
  middleware: [
    flip(),
    offset(8),
  ],
})

const computedPosition = computed(() => ({
  position: strategy.value,
  left: `${x.value ?? 0}px`,
  top: `${y.value ?? 0}px`,
}))
</script>

<template>
  <div ref="anchor" class="dropdown-wrap">
    <!-- <button class="dropdown-trigger" @click="open = true"> -->
    <slot name="button" :open="open" :trigger="() => open = !open" />
    <!-- </button> -->
    <div v-if="open" ref="dropdown" class="dropdown" :style="computedPosition">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.dropdown-trigger {
  cursor: pointer;
}
</style>
