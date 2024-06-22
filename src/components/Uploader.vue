<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { message, open } from '@tauri-apps/plugin-dialog'
import { info } from '@tauri-apps/plugin-log'
import { listen } from '@tauri-apps/api/event'

async function init() {
  await listen('upload-progress', (event) => {
    info(`Received event:${JSON.stringify(event)}`)
  })
}
init()

async function selectFiles() {
  const files = await open({
    multiple: true,
    directory: true,
  })
  if (!files) {
    await message('没有选择任何文件或文件夹', { kind: 'warning' })
    return
  }
  info(`选择文件或文件夹:${files[0]}`)
  await invoke('upload_files', { filesUri: files[0] })
}
</script>

<template>
  <div class="flex justify-center p-2">
    <div class="flex">
      <button class="iw-btn iw-btn-primary iw-btn-sm" @click="selectFiles">
        选择文件或文件夹
      </button>
    </div>
    <div id="status" />
  </div>
</template>
