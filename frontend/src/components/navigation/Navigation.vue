<script setup lang='ts'>
import Dropdown from '../generic/Dropdown.vue'
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
        <li>
          <RouterLink :to="{ name: 'RouteCreate' }">
            Create
          </RouterLink>
        </li>
        <li><a href="/">Your Projects</a></li>
      </ul>
    </nav>
    <!-- <div class="user">
      <div class="pfp">
        <strong>D</strong>
      </div>
      <span>dolanske</span>
    </div> -->
    <div class="user-wrap">
      <Dropdown>
        <template #button="{ trigger }">
          <button class="user" @click="trigger">
            <div class="pfp">
              <strong>D</strong>
            </div>
            <span>dolanske</span>
          </button>
        </template>
        <template #default>
          <p>huh</p>
        </template>
      </Dropdown>
    </div>
  </header>
</template>
