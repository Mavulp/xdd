<script setup lang='ts'>

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

    <a href="" class="user">
      <div class="pfp">
        <strong>D</strong>
      </div>
      <span>dolanske</span>
    </a>
  </header>
</template>
