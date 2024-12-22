<script lang="ts">
import {invoke} from "@tauri-apps/api/core";

export default {
  async setup() {
    const {data, error} = await useFetch("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json");
    const versions = (data.value as any)["versions"] as any[];
    const items : string[] = [];
    for (const index in versions) {
      const item = versions[index];
      if( item["type"] === "release") {
        items.push(item["id"]);
      }

    }
    const selected = ref();
    const isLoading = ref(false);
    const downloadMc = async function () {
      isLoading.value = true;
      await invoke("install_minecraft", {version: selected.value});
      isLoading.value = false;
    }
  return {
    downloadMc,
    isLoading,
    items,
    selected
  }
  }
}

</script>

<template>
  <div class="flex flex-col h-full content-center">
  Minecraft Verion:
  <USelectMenu
      searchable
      searchable-placeholder="Search a person..."
      class="w-full lg:w-48"
      placeholder="Select a person"
      :options="items"
      v-model="selected"
  />
    <UButton :loading="isLoading" :disabled="!selected" icon="i-line-md-download-twotone-loop" class="w-[90px]" @click="downloadMc" >Install</UButton>
  </div>
</template>

<style scoped>

</style>