<script setup lang="ts">
import Titlebar from "~/components/titlebar.vue";

const links = [{
  label: 'Profile',
  avatar: {
    src: 'https://avatars.githubusercontent.com/u/739984?v=4'
  },
  badge: 100
}, {
  label: 'Home',
  icon: 'i-line-md-home-alt-twotone',
  to: '/'
}, {
  label: 'Browse',
  icon: 'i-line-md-compass-loop',
  to: '/browse'
},
  {
    label: 'Minecraft',
    icon: 'i-line-md-downloading-loop',
    to: '/minecraft'
  }
]
</script>

<template>

    <div class="flex flex-row">

      <UVerticalNavigation :links="links"/>
      <slot/>
    </div>

</template>

<style scoped>

</style>