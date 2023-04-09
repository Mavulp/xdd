<script setup lang='ts'>
import Navigation from './components/navigation/Navigation.vue'
import ToastWrap from './components/toast/ToastWrap.vue'
import { useLoading } from './store/loading'
import { LOAD } from './js/definitions'
import { useUser } from './store/user'

const loading = useLoading()
// const route = useRoute()
const user = useUser()
</script>

<template>
  <div v-if="!loading.get(LOAD.APP)">
    <Navigation v-if="user.isSignedIn" />
    <RouterView />
    <ToastWrap />
  </div>
</template>
