<script setup lang="ts">
import { ref } from 'vue';
import type { Topic } from '../../db/types';

type Props = {
  topic: Topic;
  role?: string;
  className?: string;
  titleClassName?: string;

  onStartBtnClick?: (topic: Topic) => void;
  onDeleteBtnClick?: (topic: Topic) => void;
}

const isStartBtnHovered = ref(false);

const { topic, role, className, titleClassName, onStartBtnClick, onDeleteBtnClick } = defineProps<Props>();

</script>

<template>
  <div :role="role"
    :class="`relative inline-block align-top p-4 w-1/3 h-30 transition bg-blue-700 hover:scale-105 hover:shadow-lg shadow-sm shadow-blue-200 rounded-3xl ${className}`">
    <h1 :class="`single-line-text leading-none mt-1 mb-1 text-lg color-gray-100 ${titleClassName}`">{{ topic.title }}</h1>
    <p class="single-line-text mt-1 mb-10 text-sm color-blue-300">
      <small>{{ topic.desc }}</small>
    </p>
    <div v-if="topic.id" class="absolute flex justify-right items-center w-auto h-auto right-3 bottom-3 text-xs">
      <button
        class="flex items-center color-blue-200 hover:color-gray-100 p-1 hover:pr-2 h-7 rounded-xl transition bg-transparent hover:bg-blue-400 hover:scale-105"
        @click.stop="onDeleteBtnClick"
        @mouseenter="isStartBtnHovered = true"
        @mouseleave="isStartBtnHovered = false">
        <i class="mr-1 text-lg i-solar:play-circle-bold" />
        <span v-if="isStartBtnHovered">Start a quick event</span>
      </button>
      <button
        class="flex justify-center items-center p-1 bg-transparent hover:bg-pink-600 transition hover:scale-105 rounded-xl text-pink-400 hover:text-gray-100 w-8 h-7 text-lg"
        @click.stop="onStartBtnClick">
        <i class="i-solar:trash-bin-minimalistic-bold" />
      </button>

    </div>
  </div>
</template>
