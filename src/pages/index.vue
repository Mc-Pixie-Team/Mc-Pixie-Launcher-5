<script setup lang="ts">

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

</script>


<template>
 <div>
 
    

     <UHorizontalNavigation :links="links" class="border-b border-gray-200 dark:border-gray-800"></UHorizontalNavigation>
    <a href="/browse" >test</a>


  </div>
</template>

