<script setup lang="ts"></script>

<template>
  <div class="uploading-loading-icon">
    <div />
    <div />
    <div />
    <div />
    <div />
    <div />
    <div />
    <div />
  </div>
</template>

<style scoped lang="less">
.uploading-loading-icon {
  color: var(--sys-info);
  display: inline-block;
  position: relative;
  width: 12px;
  height: 12px;
  margin-left: 4px;
  & > div {
    position: absolute;
    width: 1px;
    height: 3px;
    background: var(--sys-info);
    transform-origin: 50% 6px;
    animation: loading 0.8s linear infinite;
    .loop(@i) when (@i <= 8) {
      &:nth-child(@{i}) {
        transform: rotate((@i - 1) * 45deg);
        animation-delay: ((@i * 0.1) - 0.7s);
      }
      .loop(@i + 1);
    }

    .loop(1);
  }
}

@keyframes loading {
  0% {
    opacity: 1;
  }
  100% {
    opacity: 0;
  }
}
</style>
