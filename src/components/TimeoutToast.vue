<template>
  <transition name="fade">
    <div v-if="visible" class="fixed bottom-1 right-1 bg-primary text-primary-content p-2 rounded shadow-lg z-50">
      {{ message }}
    </div>
  </transition>
</template>

<script setup>
import { ref, watchEffect } from 'vue';

const props = defineProps({
  message: {
    type: String,
    required: true
  },
  duration: {
    type: Number,
    default: 5000 // 默认显示5秒
  }
});

const visible = ref(true);

watchEffect(() => {
  const timeout = setTimeout(() => {
    visible.value = false;
  }, props.duration);

  return () => clearTimeout(timeout);
});
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>