<template>
  <div>
    <div class="container small" style="padding-top:10em;">
      <h2>Signed out!</h2>
      <p style="line-height:1.5em;">
        Hey. You are seeing this because you just logged out. The login prompt opens automatically whenever you try to access any part of the app. Except here. In case you want to log back, simply click the button.
      </p>
      <br><br>
      <div class="flex">
        <router-link class="button btn-wider btn-accent" :to="{ name: 'RouteHome' }">
          Sign In
        </router-link>
      </div>
    </div>
  </div>
</template>
