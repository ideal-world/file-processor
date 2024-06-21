<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMounted } from 'vue';
import UploaderComp from './components/Uploader.vue';

onMounted(async () => {
  const params = await invoke('get_params')
})
</script>

<template>
  <div class="text-base text-base-content bg-base-100">
    <UploaderComp />
  </div>
</template>
