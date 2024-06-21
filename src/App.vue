<script setup lang="ts">
import { onMounted } from 'vue';
import UploaderComp from './components/Uploader.vue';

onMounted(() => {
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
