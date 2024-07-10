<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { attachConsole, info } from '@tauri-apps/plugin-log'
import type { Ref } from 'vue'
import { onMounted, ref } from 'vue'
import UploaderComp from './components/Uploader.vue'

const params: Ref<FileProcessParams | null> = ref(null)
const functionContainerRef: Ref<HTMLElement | null> = ref(null)

onMounted(async () => {
  params.value = await invoke('get_params')
  info(`Init params: ${JSON.stringify(params.value)}`)
})

async function init() {
  const detach = await attachConsole()
  detach()
}
init()
</script>

<script lang="ts">
export interface FileProcessParams {
  title: string
  upload?: FileUploadProcessParams
}
export interface FileUploadProcessParams {
  target_kind_key: string
  target_obj_key: string
  overwrite: boolean
  upload_metadata_data_url: string
}
</script>

<template>
  <div class="flex flex-col justify-center items-center p-2 text-base text-base-content bg-base-100 w-full"
    style="height: 400px;">
    <div class="text-lg font-bold bg-base-200 border border-base-300 p-2 rounded-md mb-2">
      {{ params?.title }}
    </div>
    <div ref="functionContainerRef" class="flex-1 w-full overflow-hidden">
      <UploaderComp v-if="params?.upload" :upload="params.upload" />
    </div>
  </div>
</template>
