<script setup lang="ts">
import {
  onOpenUrl,
} from '@tauri-apps/plugin-deep-link';
import { message } from '@tauri-apps/plugin-dialog';
import { onMounted } from 'vue';
import UploaderComp from './components/Uploader.vue';

async function handler(urls: string[]) {
  await message(urls[0])
  const updateIntentEl = document.querySelector('#event-intent')!
  updateIntentEl.textContent = JSON.stringify(urls)
}

onMounted(() => {
  onOpenUrl(handler)
})
</script>

<template>
  <div class="text-base text-base-content bg-base-100">
    <UploaderComp />

    <div class="row">
      <p>Requested intent:</p>
      <p id="update-intent" />
    </div>

    <div class="row">
      <p>initial intent:<br></p>
      <p id="initial-intent" />
    </div>

    <div class="row">
      <p>updated intent by event:<br></p>
      <p id="event-intent" />
    </div>
  </div>
</template>
