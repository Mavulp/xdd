<script setup lang="ts">
import { IconDotsCircle } from '@iconify-prerendered/vue-mdi'
</script>

<template>
  <IconDotsCircle />
</template>

<style scoped lang="scss">
  @keyframes spin {
    from {
        transform:rotate(0deg);
    }
    to {
        transform:rotate(360deg);
    }
  }

  svg {
    animation-name: spin;
    animation-duration: 4000ms;
    animation-iteration-count: infinite;
    animation-timing-function: linear;
  }
</style>
