<script lang="ts">
import { window } from '@tauri-apps/api';


// when using `"withGlobalTauri": true`, you may use
// const { getCurrentWindow } = window.__TAURI__.window;
const appWindow = window.getCurrentWindow();

export default {
  name: 'Titlebar',
  methods: {
    minimize() {

      appWindow.minimize();
    },
    maximize() {
      appWindow.toggleMaximize();
    },
    close() {
      appWindow.close();
    }
  },
};
</script>

<template>
<!-- 
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-button">
      <img draggable="false" v-on:click="minimize" src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
    </div>
  

  <div class="titlebar-button">
    <img draggable="false" v-on:click="maximize" src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" />
  </div>

  <div class="titlebar-button" v-on:click="close">
    <img draggable="false" src="https://api.iconify.design/mdi:close.svg" alt="close" />
  </div>

</div> -->
  <slot />
</template>

<style>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  user-select: none;
  -webkit-user-select: none;
}

.titlebar-button:hover {
  background: #5bbec3;
}
</style>