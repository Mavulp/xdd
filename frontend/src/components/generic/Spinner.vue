<template>
  <!-- <span> -->
  <Icon icon="ph:spinner-bold" />
  <!-- </span> -->
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
