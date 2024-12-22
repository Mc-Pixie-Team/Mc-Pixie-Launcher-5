<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
export default {
  setup() {


  },
  methods: {
    downloadFile: function () {
      invoke('my_custom_command', { target: 'https://upload.wikimedia.org/wikipedia/commons/thumb/1/11/Test-Logo.svg/783px-Test-Logo.svg.png' }).then(() =>
          console.log('Completed!')
      );
    }
  }
}

</script>


<template >

 <div >
   <UDropdown :items="[[{ label: 'Edit', shortcuts: ['E'] }]]" />

   <h1 class="dark:text-cyan-500">
     Hello world!
   </h1>
  <UButton @click="downloadFile">
    Toggle
  </UButton>
   <UCard>
     <template #header>
       <Placeholder class="h-8" />
     </template>

     <Placeholder class="h-32" />

     <template #footer>
       <Placeholder class="h-8" />
     </template>
   </UCard>
   Primary: {{ useAppConfig().ui.primary }}
   Gray: {{ useAppConfig().ui.gray }}
  </div>
  <a href="/browse">browse</a>

</template>

