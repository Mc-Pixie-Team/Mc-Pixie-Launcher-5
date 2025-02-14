<script setup lang="ts">
import Titlebar from "~/components/titlebar.vue";

const items = ref([
  {
    label: 'Providers',
    icon: 'i-lucide-compass',
    to: '/',
    open: true,
    children: [
      {
        label: 'Modrinth',
        to: '/browse-Modrinth'
      },
      {
        label: 'Curseforge',
        to: '/browse-Curseforge'
      },
      {
        label: 'FTB',
        to: '/browse-FTB'
      },
      {
        label: 'Pixie',
        to: '/browse-Pixie'
      },
    ]
  },
  {
    label: 'Home',
    icon: 'i-lucide-house',
    to: '/',

  },
  {
    label: 'Settings',
    icon: 'i-lucide-settings',
    to: '/settings',

  },

])
</script>

<template>
  <div class="w-full h-screen flex flex-col">
    <Titlebar class="min-h-[38px]" /> 
    <div class="w-full flex flex-row " style="height: calc(100% - 38px);">

      <UNavigationMenu :items="items" highlight color="neutral" highlight-color="primary" orientation="vertical"
        class="w-[180px] mx-2.5">
      </UNavigationMenu>

      <div class="mb-10 mr-2.5" >
        <USeparator orientation="vertical"  icon="i-lucide-house" />
      </div>
      <div class="flex-1">
        <slot />
      </div>
    </div>

  </div>
</template>
