<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { useVirtualList } from '@vueuse/core'



// Track pagination and loading state
const page = ref(0)
const allItems = ref([])
const hasMoreItems = ref(true)
const isFetchingMore = ref(false)

// Initial data fetch with useLazyFetch
const { status: initialStatus, data: initialData, refresh } = await useLazyFetch('https://api.curseforge.com/v1/mods/search', {
  params: {
    pageSize: 20,
    gameId: 432,
    index: 0
  },
  headers: {
    "x-api-key": "$2a$10$zApu4/n/e1nylJMTZMv5deblPpAWUHXc226sEIP1vxCjlYQoQG3QW"
  }
})

// Process initial data when it arrives
watch(initialData, (newData) => {
  if (newData && newData.data) {
    allItems.value = newData.data
  }
}, { immediate: true })

// Function to load more items
const loadMoreItems = async () => {
  if (isFetchingMore.value || !hasMoreItems.value) return
  
  isFetchingMore.value = true
  const nextPage = page.value + 1
  
  try {

    const { data: moreData } = await useLazyFetch('https://api.curseforge.com/v1/mods/search', {
      params: {
        pageSize: 20,
        gameId: 432,
        index: nextPage
      },
      headers: {
        "x-api-key": "$2a$10$zApu4/n/e1nylJMTZMv5deblPpAWUHXc226sEIP1vxCjlYQoQG3QW"
      },
    })

    if (moreData.value && moreData.value.data && moreData.value.data.length > 0) {
      allItems.value = [...allItems.value, ...moreData.value.data]
      page.value = nextPage
    } else {
      hasMoreItems.value = false
    }
  } catch (error) {
    console.error('Error loading more items:', error)
    hasMoreItems.value = false
  } finally {
    isFetchingMore.value = false
  }
}

// Virtual list configuration
const { list, containerProps, wrapperProps } = useVirtualList(
  allItems,
  {
    itemHeight: 170,
    overscan: 10
  }
)

// Watch the last visible items to trigger loading more when needed
const loadMoreThreshold = 5 // Load more when 5 items from the end
watch(list, (newList) => {
  if (!newList.length) return
  
  const lastVisibleIndex = newList[newList.length - 1].index
  if (lastVisibleIndex >= allItems.value.length - loadMoreThreshold && !isFetchingMore.value && hasMoreItems.value) {
    loadMoreItems()
  }
})
</script>

<template>
  <div class="h-full w-full flex flex-col overflow-hidden">
    <div class="text-xs font-medium text-[var(--ui-text-muted)]">Modpack Provider:</div>
    <div class="text-3xl font-normal text-[var(--ui-text-highlighted)]">{{ $route.params.id }}</div>
    
    <!-- Initial loading state -->
    <div v-if="initialStatus === 'pending' && allItems.length === 0" class="w-full h-full flex justify-center items-center mt-10">
      Loading...
    </div>
    
    <!-- Virtual list -->
    <div v-else-if="allItems.length > 0" v-bind="containerProps" class="w-full h-full mt-10" style="overflow-y: scroll!important;">
      <div v-bind="wrapperProps">
        <div v-for="item in list" :key="item.index" class="h-[170px] mr-3.5">
          <Card :project-data="item.data" />
        </div>
        
        <!-- Loading more indicator -->
        <div v-if="isFetchingMore" class="py-4 text-center">
          Loading more items...
        </div>
        
        <!-- End of list message -->
        <div v-if="!hasMoreItems && !isFetchingMore" class="py-4 text-center text-gray-500">
          No more items to load
        </div>
      </div>
    </div>
    
    <!-- No items found -->
    <div v-else-if="initialStatus === 'success' && allItems.length === 0" class="w-full h-full flex justify-center items-center mt-10">
      No items found
    </div>
  </div>
</template>
