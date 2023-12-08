<script setup lang="ts">
import { computed } from 'vue';

type Props = {
  className?: string;
  state?: 'default' | 'danger' | 'highlight'
};

const { className, state = 'default' } = defineProps<Props>();
const stateClass = computed(() => {
  switch (state) {
    case 'default':
      return 'text-blue-300 hover:text-blue-500 hover:bg-gray-200';
    case 'danger':
      return 'text-pink-400 hover:text-gray-100 hover:bg-pink-600';
    case 'highlight':
      return 'text-blue-200 hover:text-gray-100 hover:bg-blue-400'
  }
});

</script>

<template>
<button
  :class="`flex justify-center items-center p-1 h-7 rounded-xl bg-transparent transition hover:scale-105 ${className} ${stateClass}`"
>
  <slot name="icon"></slot>
  <slot></slot>
</button>
</template>

<style scoped>
</style>