<script setup lang='ts'>
import Navigation from './components/navigation/Navigation.vue'
import ToastWrap from './components/toast/ToastWrap.vue'
</script>

<template>
  <div>
    <Navigation />
    <RouterView />
    <ToastWrap />
  </div>
</template>
