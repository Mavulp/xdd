<script setup lang='ts'>
import { useToast } from '../../store/toast'
import ToastItem from './ToastItem.vue'
const toasts = useToast()
</script>

<template>
  <div class="toasts-wrap">
    <ul>
      <TransitionGroup name="toast">
        <ToastItem v-for="[, item] in toasts.items" :key="item._id" :data="item" />
      </TransitionGroup>
    </ul>
  </div>
</template>

<style>
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  margin-bottom: 0;
  height: 0;
  line-height: 0;
}
</style>
