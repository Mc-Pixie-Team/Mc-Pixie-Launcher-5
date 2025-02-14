<script lang="ts">
import { UButton } from '#components';
import { invoke } from '@tauri-apps/api/core';

let manifest : MinecraftManifest;

type MinecraftManifest = {
  "latest": {
    "release": string,
    "snapshot": string
  },
  "versions": MinecraftVersion[],
  "versionIds": string[]
}
type MinecraftVersion = {
  "id": string,
  "type": string,
  "url": string,
  "time": string,
  "releaseTime": string,
  "sha1": string,
  "complianceLevel": number
}

export default {
  data(): { value: any; data: string[]; loading: boolean } {
    return {
      value: '',
      data: [],
      loading: false,
    };
  },
  methods: {
    async invokeMetadata() {
      if (this.data.length) return;
      this.loading = true;
      try {
        manifest = await invoke('get_minecraft_metadata')
        this.data = manifest.versionIds;
      } catch (e) {
        console.error(e)
      }
      this.loading = false;
    },
    async invokeGameInstall() {
    console.log(manifest.versions.find(version => version.id === this.value))
    }
  },
};
</script>

<template>
  <div class="inline-flex flex-col">
  <USelectMenu @update:open="invokeMetadata" :loading="loading" placeholder="Select mc version..." v-model="value"
    :items="data" class="w-48" />
  <UButton @click="invokeGameInstall"  size="md" color="primary" label="Install" variant="subtle" icon="i-lucide-download" />
</div>
</template>