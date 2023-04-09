<script setup lang='ts'>
import { useRouter } from 'vue-router'
import { TOKEN_KEY } from '../../js/config'
import { post } from '../../js/fetch'
import { useUser } from '../../store/user'
import Dropdown from '../generic/Dropdown.vue'

const user = useUser()
const router = useRouter()

async function signOut() {
  // Call the logout endpint
  // Proceed further even if it fails
  await post('/auth/logout', '').catch(() => {})

  localStorage.removeItem(TOKEN_KEY)
  document.cookie = '__auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT;'
  user.reset()
  router.push({ name: 'RouteSignOut' })
}
</script>

<template>
  <header>
    <RouterLink :to="{ name: 'RouteHome' }" class="disable-hover">
      <img src="/icons/logo.svg" alt="">
    </RouterLink>

    <nav>
      <ul>
        <li>
          <RouterLink :to="{ name: 'RouteHome' }">
            Aliases
          </RouterLink>
        </li>
        <li v-if="user.can('create-aliases')">
          <RouterLink :to="{ name: 'RouteCreate' }">
            Create
          </RouterLink>
        </li>
        <!-- <li><a href="/">Your Projects</a></li> -->
      </ul>
    </nav>
    <div class="user-wrap">
      <Dropdown>
        <template #button="{ trigger }">
          <button class="user" @click="trigger">
            <div class="pfp">
              <strong>{{ user.username.at(0) }}</strong>
            </div>
            <span>{{ user.username }}</span>
          </button>
        </template>
        <template #default>
          <button class="button btn-white btn-small btn-full" @click="signOut()">
            Sign Out
          </button>
        </template>
      </Dropdown>
    </div>
  </header>
</template>
