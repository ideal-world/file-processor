<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import type { FileResponse } from '@tauri-apps/plugin-dialog'
import { message, open } from '@tauri-apps/plugin-dialog'
import { debug, info } from '@tauri-apps/plugin-log'
import { nextTick, ref } from 'vue'
import { exit } from '@tauri-apps/plugin-process'
import type { FileUploadProcessParams } from '../App.vue'
import FileList from './FileList.vue'

const props = defineProps<{
  upload: FileUploadProcessParams
}>()

const totalStatsResp = ref<UploadStatsResp | null>(null)
const uploadedStatsResp = ref<UploadStatsResp | null>(null)
const triggerUpload = ref<boolean>(false)
const isDone = ref<boolean>(false)
const failOnly = ref<boolean>(false)
const progress = ref<string>('0')
const uploaded_file_numbers = ref<number>(0)

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
  debug(`totalStatsResp.value :${JSON.stringify(totalStatsResp.value)}`)
}
function handleFileListIsDone() {
  isDone.value = true
}
function handleUpdateProgress(value: number) {
  uploaded_file_numbers.value = value
  progress.value = ((value / totalStatsResp.value!.total_file_numbers) * 100).toFixed(2)
}
async function exit0() {
  await exit(0)
}

async function cancel() {
  await invoke('cancel')
  isDone.value = true
}
</script>

<script lang="ts">
export interface UploadProgressResp {
  uploaded_file_numbers: number
  uploaded_file_size: number
  current_files: UploadFileInfo[]
  success_files: UploadFileInfo[]
  fail_files: UploadFileInfo[]
}
export interface UploadFileInfo {
  id: string
  name: string
  relative_path: string
  size: number
}
export interface UploadStatsResp {
  total_file_numbers: number
  total_file_size: number
}
</script>

<template>
  <div v-if="!totalStatsResp" class="flex flex-col h-full w-full">
    <span class="font-bold justify-start">请选择上传方式:</span>
    <div class="flex flex-col h-full w-full justify-center items-center">
      <template v-if="!triggerUpload">
        <div class="flex justify-center items-center">
          <button class="iw-btn iw-btn-primary self-center w-28 h-[98px] py-2" @click="selectFiles(false)">
            <svg t="1723100020811" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
              p-id="4234" width="50" height="50">
              <path
                d="M288 320a32 32 0 1 1 0-64h448a32 32 0 0 1 0 64H288zM288 544a32 32 0 0 1 0-64h448a32 32 0 0 1 0 64H288zM288 768a32 32 0 0 1 0-64h128a32 32 0 0 1 0 64H288z"
                fill="currentColor" p-id="4235" />
              <path
                d="M807.968 802.24a32 32 0 0 1 41.984 48.32l-153.856 133.6a32 32 0 0 1-20.992 7.84H195.04C140.16 992 96 946.624 96 891.072V132.928C96 77.376 140.16 32 195.04 32h633.92C883.84 32 928 77.376 928 132.928v564.32a32 32 0 1 1-64 0V132.928C864 112.32 848.096 96 828.96 96H195.04C175.904 96 160 112.32 160 132.928v758.144C160 911.68 175.904 928 195.04 928h468.096l144.832-125.76z"
                fill="currentColor" p-id="4236" />
              <path
                d="M704 959.136a32 32 0 1 1-64 0v-186.24C640 717.408 684.16 672 739.04 672h157.632a32 32 0 0 1 0 64h-157.632c-19.136 0-35.04 16.32-35.04 36.928v186.24z"
                fill="currentColor" p-id="4237" />
            </svg>
            <span>文件上传</span>
          </button>
          <button class="iw-btn iw-btn-primary self-center w-28 h-[98px] ml-12" @click="selectFiles(true)">
            <svg class="icon" viewBox="0 0 1100 1100" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="4449"
              width="50" height="50">
              <path
                d="M912 208H427.872l-50.368-94.176A63.936 63.936 0 0 0 321.056 80H112c-35.296 0-64 28.704-64 64v736c0 35.296 28.704 64 64 64h800c35.296 0 64-28.704 64-64v-608c0-35.296-28.704-64-64-64z m-800-64h209.056l68.448 128H912v97.984c-0.416 0-0.8-0.128-1.216-0.128H113.248c-0.416 0-0.8 0.128-1.248 0.128V144z m0 736v-96l1.248-350.144 798.752 1.216V784h0.064v96H112z"
                fill="currentColor" p-id="4450" />
            </svg>
            <span>文件夹上传</span>
          </button>
        </div>
        <!-- <span class="text-sm mt-4">文件冲突处理：{{ props.upload.overwrite ? "覆盖" : "跳过" }}</span> -->
      </template>
      <template v-else>
        <div class="flex flex-col justify-center items-center h-full w-full">
          <div class="flex flex-col justify-center items-center">
            <span class="text-lg">上传处理中...</span>
          </div>
        </div>
      </template>
    </div>
  </div>
  <div v-else class="flex flex-col h-full w-full">
    <div class="w-full">
      <progress class="iw-progress iw-progress-primary w-full h-6 rounded-full " :value="progress" max="100" />
      <span className=" relative -top-8 text-white text-sm font-medium" style="left: 46%">
        {{ progress }}%
      </span>
    </div>
    <div class="flex justify-center p-1 border-b border-b-base-300 items-center -mt-8">
      <span class="font-bold" title="已上传文件数"> {{ uploaded_file_numbers }}</span> / <span class="font-bold" title="总文件数">
        {{ totalStatsResp!.total_file_numbers }}</span> &nbsp; | &nbsp;
      <!-- <span class="font-bold" title="已上传大小"> {{ (uploadedStatsResp!.total_file_size / 1024 / 1024).toFixed(2) }}</span>
      /
      <span class="font-bold" title="总大小"> {{ (totalStatsResp!.total_file_size / 1024 / 1024).toFixed(2) }}</span> MB -->
      <label class="label cursor-pointer flex justify-center items-center">
        <span class="label-text">只看失败</span>
        <input v-model="failOnly" type="checkbox" class="iw-toggle iw-toggle-primary iw-toggle-sm">
      </label>
    </div>
    <FileList :fail-only="failOnly" :total-stats-resp="totalStatsResp" @is-done="handleFileListIsDone"
      @update-progress="handleUpdateProgress" />
    <button v-if="!isDone" class="iw-btn iw-btn-accent iw-glass iw-btn-sm self-center" @click="cancel">
      中止
    </button>
    <button v-if="isDone" class="iw-btn iw-btn-success iw-btn-sm self-center" @click="exit0">
      完成
    </button>
  </div>
</template>

<style></style>
