<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

// Props definition
const props = defineProps({
  projectData: {
    type: Object,
    required: true
  }
});

let projectJson = JSON.parse(JSON.stringify(props.projectData))


const confirm = async () => {
  try {
    const requestdata = {
      name: projectJson['name'],
      iconUri: projectJson['logo']['thumbnailUrl'] || '', // Optional chaining for safety
      provider: 'Pixie', // Assuming 'Pixie' is a string or constant
      rawProjectData: projectJson
    };


    
    await invoke('install_modpack', { firstRequestData: requestdata });
  } catch (e) {
    console.error('Error:', e);
  }
};
</script>

<template>
  <div class="h-[147px] rounded-xl border-[var(--ui-border)] border p-[18px]">
    <div id="header" class="flex flex-row ">
      <img
          :src="projectJson['logo']['thumbnailUrl']"
          class="h-[111px] w-auto rounded-lg"
      />
      <div class="w-full"></div>
      <UButton v-on:click="confirm" class="self-end justify-self-end"  size="md" color="primary" label="Install" variant="subtle" icon="i-lucide-download" />
      

      
    </div>
  </div>
</template>

<style scoped>

</style>