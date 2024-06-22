<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { onMounted } from 'vue'
import { attachConsole, info } from '@tauri-apps/plugin-log'
import UploaderComp from './components/Uploader.vue'

onMounted(async () => {
  const params = await invoke('get_params')
  info(`Init frontend:${JSON.stringify(params)}`)
})

async function init() {
  const detach = await attachConsole()
  detach()
}
init()
</script>

<template>
  <div class="text-base text-base-content bg-base-100">
    <UploaderComp />
  </div>
</template>
