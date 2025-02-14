<script setup>
import { useVirtualList } from '@vueuse/core'
import Card from '~/components/card.vue';

const { data, error } = await useFetch('https://api.curseforge.com/v1/mods/search?pageSize=50&gameId=432&index=0', {
  headers: {
    "x-api-key": "$2a$10$zApu4/n/e1nylJMTZMv5deblPpAWUHXc226sEIP1vxCjlYQoQG3QW"
  }
})

if (error.value) throw error.value;


const { list, containerProps, wrapperProps } = useVirtualList(
  data.value["data"],
  {
    itemHeight: 170,
  },)
</script>

<template>
  <div class="h-full w-full flex flex-col">
    <div class="text-xs font-medium text-[var(--ui-text-muted)]">Modpack Provider:</div>
    <div class="text-3xl font-medium">{{ $route.params.id }}</div>

      <div v-bind="containerProps" class="w-full h-full mt-10" style="overflow-y: scroll!important;">
        <div v-bind="wrapperProps">
          <div v-for="item in list" :key="item.index" class="h-[170px] mr-3.5">
            <Card :project-data="item.data" />
          </div>
        </div>
      </div>
  
  </div>
</template>
