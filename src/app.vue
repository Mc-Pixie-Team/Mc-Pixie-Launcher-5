<script setup>
import { onMounted } from 'vue';
import gsap from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';
import { ScrollToPlugin } from 'gsap/ScrollToPlugin';


gsap.registerPlugin(ScrollTrigger, ScrollToPlugin);


</script>

<template>
  <div class="h-screen w-screen overflow-hidden">
  <UApp>
    
    <NuxtLayout  >
      <NuxtPage :transition="transitionConfig"/>
    </NuxtLayout>
  
  </UApp>
</div>
</template>
 