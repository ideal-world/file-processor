<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { attachConsole, info } from '@tauri-apps/plugin-log'
import type { Ref } from 'vue'
import { onMounted, ref } from 'vue'
import UploaderComp from './components/Uploader.vue'
import Toast from './components/TimeoutToast.vue';

const params: Ref<FileProcessParams | null> = ref(null)
const functionContainerRef: Ref<HTMLElement | null> = ref(null)
const showToastMessage = ref(false);
const toastMessage = ref('');

onMounted(async () => {
  params.value = await invoke('get_params')
  info(`Init params: ${JSON.stringify(params.value)}`)
  let version: string = await invoke('get_version')
  if (version !== params.value?.upload?.target_version) {
    toastMessage.value = `您的版本可能和最新版本不一致,现在版本${version},目标版本${params.value?.upload?.target_version}`;
    showToastMessage.value = true;
  }
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
  target_version: string
  upload_metadata_data_url: string
}
</script>

<template>
  <div class="flex flex-col justify-center items-center p-2 text-base text-base-content bg-base-100 w-full"
    style="height: 400px;">
    <div
      class="text-lg w-full flex flex-col justify-center items-center font-bold bg-base-200 border border-base-300 p-2 rounded-md mb-2">
      <span>
        {{ params?.title }}
      </span>
    </div>
    <Toast v-if="showToastMessage" :message="toastMessage" @close="showToastMessage = false" />
    <div ref="functionContainerRef" class="flex-1 w-full overflow-hidden">
      <UploaderComp v-if="params?.upload" :upload="params.upload" />
    </div>
  </div>
</template>
