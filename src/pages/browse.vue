<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

const links = [ {
  label: 'Home',
  icon: 'i-heroicons-home',
  to: '/'
}, {
  label: 'Browse',
  icon: 'i-heroicons-chart-bar',
  to: '/browse'
}, {
  label: 'Settings',
  icon: 'i-heroicons-command-line',
  to: '/settings/settings'
},
{
  
  label: 'Profile',
  avatar: {
    src: 'https://avatars.githubusercontent.com/u/739984?v=4'
  },
  badge: 100,
  to: '/profile/profile'
}
]
function doInvoke() {
  console.log("test")
 invoke("my_custom_command", {target: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQQrGU-rWRbOjdT3tmvwow9kRwR1ovtXboW2A&s"})
}
</script>

<template>
  <div >
    <UHorizontalNavigation :links="links" class="border-b border-gray-200 dark:border-gray-800"></UHorizontalNavigation>
    Browse

    <UButton v-on:click='doInvoke' >Button</UButton>
  </div>
</template>