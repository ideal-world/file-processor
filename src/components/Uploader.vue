<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { FileResponse } from '@tauri-apps/plugin-dialog'
import { message, open } from '@tauri-apps/plugin-dialog'
import { debug, info } from '@tauri-apps/plugin-log'
import { nextTick, ref } from 'vue'
import type { FileUploadProcessParams } from '../App.vue'

const props = defineProps<{
  upload: FileUploadProcessParams
}>()

const totalStatsResp = ref<UploadStatsResp | null>(null)
const uploadedStatsResp = ref<UploadStatsResp | null>(null)
const progressRef = ref<HTMLElement>()
const triggerUpload = ref<boolean>(false)
const isUserScrolling = ref<boolean>(false)

async function init() {
  await listen('upload-progress', (event) => {
    debug(`Received event:${JSON.stringify(event)}`)
    const progressResp = event.payload as UploadProgressResp
    uploadedStatsResp.value = {
      total_file_numbers: progressResp.uploaded_file_numbers,
      total_file_size: progressResp.uploaded_file_size,
    }
    progressRef.value!.querySelectorAll('.uploading').forEach((el) => {
      el.classList.remove('uploading')
      el.childNodes[1].textContent = '已上传'
    })
    progressResp.current_files.forEach((file) => {
      const fileDiv = document.createElement('div')
      fileDiv.innerHTML = `<span class='uploading'>${file.full_name} (${(file.size / 1024).toFixed(2)}KB) &nbsp; <i>上传中...</i></span>`
      progressRef.value!.appendChild(fileDiv)
      if (!isUserScrolling.value) {
        progressRef.value!.scrollTop = progressRef.value!.scrollHeight
      }
    })
  })
}
init()

async function selectFiles(is_dir: boolean) {
  const files = await open({
    multiple: true,
    directory: is_dir,
  })
  if (!files) {
    await message('没有选择任何文件或文件夹', { kind: 'warning' })
    return
  }
  triggerUpload.value = true
  const filesUri = is_dir ? files : files.map((v: FileResponse) => v.path)
  info(`upload file from :${JSON.stringify(filesUri)}`)
  uploadedStatsResp.value = {
    total_file_numbers: 0,
    total_file_size: 0,
  }
  totalStatsResp.value = await invoke('upload_files', { filesUris: filesUri })
  nextTick(() => {
    listenScroll()
  })
}

function listenScroll() {
  progressRef.value!.addEventListener('scroll', () => {
    if (progressRef.value!.scrollTop + progressRef.value!.clientHeight >= progressRef.value!.scrollHeight - 10) {
      isUserScrolling.value = false
    }
    else {
      isUserScrolling.value = true
    }
  })
}
</script>

<script lang="ts">
export interface UploadProgressResp {
  uploaded_file_numbers: number
  uploaded_file_size: number
  current_files: UploadFileInfo[]
}
export interface UploadFileInfo {
  name: string
  full_name: string
  size: number
}
export interface UploadStatsResp {
  total_file_numbers: number
  total_file_size: number
}
</script>

<template>
  <div v-if="!totalStatsResp" class="flex flex-col justify-center items-center h-full w-full">
    <template v-if="!triggerUpload">
      <button class="iw-btn iw-btn-primary self-center w-28" @click="selectFiles(false)">
        <span>选择文件</span>
      </button>
      <button class="iw-btn iw-btn-primary self-center w-28 mt-1" @click="selectFiles(true)">
        <span>选择文件夹</span>
      </button>
      <span class="text-sm mt-1">文件冲突处理：{{ props.upload.overwrite ? "覆盖" : "跳过" }}</span>
    </template>
    <template v-else>
      <div class="flex flex-col justify-center items-center h-full w-full">
        <div class="flex flex-col justify-center items-center">
          <span class="text-lg">上传处理中...</span>
        </div>
      </div>
    </template>
  </div>
  <div v-else class="flex flex-col h-full w-full">
    <div class="flex justify-center p-1 border-b border-b-base-300">
      <span class="font-bold" title="已上传文件数"> {{ uploadedStatsResp!.total_file_numbers }}</span> / <span
        class="font-bold" title="总文件数"> {{ totalStatsResp!.total_file_numbers }}</span> &nbsp; | &nbsp;
      <span class="font-bold" title="已上传大小"> {{ (uploadedStatsResp!.total_file_size / 1024 / 1024).toFixed(2) }}</span>
      /
      <span class="font-bold" title="总大小"> {{ (totalStatsResp!.total_file_size / 1024 / 1024).toFixed(2) }}</span> MB
    </div>
    <div ref="progressRef" class="flex-1 overflow-auto text-sm" />
  </div>
</template>

<style>
.uploading {
  @apply text-primary;

  i {
    @apply text-info;
  }
}
</style>
