<script lang="ts">
export default {
  props: {
    title: String,
    icon_url: String
  }
}
</script>

<template>
  <div class="h-[130px] mb-[20px] rounded-xl border-gray-700 border mx-4 px-4 content-center">
    <div id="header" class="flex flex-row ">
      <img
          :src="icon_url"
          class="h-[100px] w-[100px] rounded-lg"
      />
      <h1 class="text-xl mt-3 font-medium flex-grow ml-3">
        {{ title }}
      </h1>
      <UButton
          class="ml-auto self-end"
          icon="i-quill-download"
          size="sm"
          color="primary"
          variant="solid"
          label="Button"
          :trailing="false"
      />
    </div>
  </div>
</template>

<style scoped>

</style>