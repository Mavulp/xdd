<script setup lang='ts'>
import Navigation from './components/navigation/Navigation.vue'
import ToastWrap from './components/toast/ToastWrap.vue'
import { useLoading } from './store/loading'
import { LOAD } from './js/definitions'

const loading = useLoading()
</script>

<template>
  <div v-if="!loading.get(LOAD.APP)">
    <Navigation />
    <RouterView />
    <ToastWrap />
  </div>
</template>
