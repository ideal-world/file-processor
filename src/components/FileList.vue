<script setup lang="ts">
import { defineEmits, defineProps, nextTick, onMounted, reactive, ref, watch } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { debug } from '@tauri-apps/plugin-log'
import type { UploadProgressResp, UploadStatsResp } from './Uploader.vue'

const props = defineProps<{
  failOnly: boolean
  totalStatsResp: UploadStatsResp
}>()
const emit = defineEmits<{
  (e: 'isDone'): void
  (e: 'updateProgress', value: number): void
}>()

const fileList = ref<FileInfo[]>([])
const uploadedStatsResp = ref<UploadStatsResp>({ total_file_numbers: 0, total_file_size: 0 })

async function init() {
  await listen<UploadProgressResp>('upload-progress', (event: { payload: UploadProgressResp }) => {
    const progressResp = event.payload as UploadProgressResp
    debug(`Received progressResp:${JSON.stringify(progressResp)}`)
    debug(`totalStatsResp.value:${JSON.stringify(props.totalStatsResp)}`)
    uploadedStatsResp.value = {
      total_file_numbers: progressResp.uploaded_file_numbers,
      total_file_size: progressResp.uploaded_file_size,
    }
    if (progressResp.uploaded_file_numbers === props.totalStatsResp.total_file_numbers) {
      emit('isDone')
    }
    emit('updateProgress', progressResp.uploaded_file_numbers)
    const fileMap = new Map<string, FileInfo>()

    fileList.value.forEach((file: FileInfo) => {
      fileMap.set(file.id, file)
    })

    progressResp.success_files.forEach((info) => {
      if (fileMap.has(info.id)) {
        fileMap.get(info.id)!.stat = UploadStat.Success
      }
    })

    progressResp.fail_files.forEach((info) => {
      if (fileMap.has(info.id)) {
        fileMap.get(info.id)!.stat = UploadStat.Fail
      }
    })

    progressResp.current_files.forEach((info) => {
      if (!fileMap.has(info.id)) {
        fileMap.set(info.id, {
          stat: UploadStat.Uploading,
          ...info,
        })
      }
    })

    fileList.value = Array.from(fileMap.values())
    debug(`fileList.value: ${JSON.stringify(fileList.value)}`)
  })
}

onMounted(() => {
  init()
})

watch(fileList, () => {
  nextTick(() => {
    scrollToBottom()
  })
})

function formatFileSize(size: number) {
  if (size < 1024) {
    return `${size.toFixed(2)}B`
  }
  else if (size < 1024 * 1024) {
    return `${(size / 1024).toFixed(2)}KB`
  }
  else if (size < 1024 * 1024 * 1024) {
    return `${(size / (1024 * 1024)).toFixed(2)}MB`
  }
  else {
    return `${(size / (1024 * 1024 * 1024)).toFixed(2)}GB`
  }
}
</script>

<script lang="ts">
export interface FileInfo {
  id: string
  name: string
  relative_path: string
  size: number
  stat: UploadStat
}

enum UploadStat {
  Uploading,
  Success,
  Fail,
}
</script>

<template>
  <div class="flex-1 overflow-auto text-sm">
    <div
      v-for="file in fileList" :key="file.id"
    >
      <div
        v-if="!props.failOnly || (props.failOnly && file.stat === UploadStat.Fail)" :id="file.id" class="flex bg-gray-200 border-2 border-dashed bg-gary-50 rounded-lg my-1" :class="[{
          'upload-success': file.stat === UploadStat.Success,
          'upload-fail': file.stat === UploadStat.Fail,
          'upload': file.stat === UploadStat.Uploading,
        }]"
      >
        <div class="truncate w-0 flex flex-grow">
          <span class="flex-shrink truncate">{{ file.relative_path }}</span>
          <div class="ml-2 flex-shrink-0 flex items-center min-w-[1rem]">
            <svg v-if="file.stat === UploadStat.Success" t="1723532907858" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="10322" xmlns:xlink="http://www.w3.org/1999/xlink" width="1rem" height="1rem"><path d="M512 1024a512 512 0 1 1 512-512 512.576 512.576 0 0 1-512 512zM269.504 451.2l-68.864 57.6 268.8 285.312a1732.992 1732.992 0 0 1 371.456-536.832l-16.384-39.68a1751.04 1751.04 0 0 0-399.744 360.832z" fill="currentColor" fill-opacity="0.85" p-id="10323" /></svg>
            <svg v-if="file.stat === UploadStat.Fail" t="1723532920422" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="10465" xmlns:xlink="http://www.w3.org/1999/xlink" width="1rem" height="1rem"><path d="M511.872032 1023.744064a511.872032 511.872032 0 1 1 511.872032-511.872032 511.872032 511.872032 0 0 1-511.872032 511.872032z m0-584.173957L367.204199 294.774306 294.774306 367.204199 439.570107 511.872032l-144.795801 144.923769L367.204199 729.417646 511.872032 584.365909l144.923769 145.051737 72.621845-72.621845L584.365909 511.872032 729.417646 367.204199l-72.621845-72.429893z" fill="currentColor" fill-opacity="0.85" p-id="10466" /></svg>
            <svg v-if="file.stat === UploadStat.Uploading" t="1723532960000" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="10700" xmlns:xlink="http://www.w3.org/1999/xlink" width="1rem" height="1rem">
              <path d="M512 64C265.6 64 64 265.6 64 512c0 38.4 6.4 76.8 12.8 108.8 6.4 38.4 44.8 64 83.2 57.6s64-44.8 57.6-83.2c-6.4-28.8-12.8-57.6-12.8-83.2 0-198.4 160-358.4 358.4-358.4s358.4 160 358.4 358.4-160 358.4-358.4 358.4c-38.4 0-76.8-6.4-108.8-12.8-38.4-6.4-76.8 19.2-83.2 57.6s19.2 76.8 57.6 83.2c38.4 6.4 76.8 12.8 121.6 12.8 256 0 448-192 448-448S768 64 512 64z" fill="currentColor" fill-opacity="0.85" p-id="10701">
                <animateTransform attributeName="transform" type="rotate" from="0 512 512" to="360 512 512" dur="1s" repeatCount="indefinite" />
              </path>
            </svg>
          </div>
        </div>
        <div class="ml-auto flex-shrink-1">
          {{ formatFileSize(file.size) }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.uploading {
  @apply text-primary;
}

.upload-success {
  @apply text-success;
}

.upload-fail {
  @apply text-error;
}
</style>
