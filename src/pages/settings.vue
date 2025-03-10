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
  setup() {

  },
  data(): { value: any; data: string[]; loading: boolean, items: any } {
    return {
      value: '',
      data: [],
      loading: false,
      items: ref([
  {
    label: 'app/',
    defaultExpanded: true,
    children: [
      {
        label: 'composables/',
        children: [
          {
            label: 'useAuth.ts',
            icon: 'i-vscode-icons-file-type-typescript'
          },
          {
            label: 'useUser.ts',
            icon: 'i-vscode-icons-file-type-typescript'
          }
        ]
      },
      {
        label: 'components/',
        defaultExpanded: true,
        children: [
          {
            label: 'Card.vue',
            icon: 'i-vscode-icons-file-type-vue'
          },
          {
            label: 'Button.vue',
            icon: 'i-vscode-icons-file-type-vue'
          }
        ]
      }
    ]
  },
  {
    label: 'app.vue',
    icon: 'i-vscode-icons-file-type-vue'
  },
  {
    label: 'nuxt.config.ts',
    icon: 'i-vscode-icons-file-type-nuxt'
  }
])
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
  <UTree :items="items">

  </UTree>
</div>
</template>