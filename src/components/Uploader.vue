<script setup lang="ts">
import { message, open } from '@tauri-apps/plugin-dialog'

async function selectFiles() {
  await message('File not found', { title: 'Tauri', type: 'error' })
  alert('ssss')
  const files = await open({
    multiple: true,
    directory: true,
  })
  console.log(files)
}
</script>

<template>
  <div class="flex">
    <button class="iw-btn" @click="selectFiles">
      选择文件或文件夹
    </button>
    <div id="status" />
  </div>
</template>
