<script setup lang="ts">
import { ref } from 'vue';

import Card from '../base/Card.vue';
import type { Topic } from '../../db/types';

type Props = {
  topic: Topic;
  className?: string;
  titleClassName?: string;
}

type Events = {
  (e: 'click:card', payload: Topic): void;
  (e: 'click:btn:start', payload: Topic): void;
  (e: 'click:btn:delete', payload: Topic): void;
}

const isStartBtnHovered = ref(false);

const { topic, className, titleClassName } = defineProps<Props>();
const emit = defineEmits<Events>();
</script>

<template>
  <Card
    role="button"
    @click="emit('click:card', topic)"
    :class-name="className"
    :title-class-name="titleClassName">
    <template #title>
      {{ topic.title }}
    </template>
    <template #desc>
      {{ topic.desc }}
    </template>
    <template #footer>
      <button
        class="flex items-center color-blue-200 hover:color-gray-100 p-1 hover:pr-2 h-7 rounded-xl transition bg-transparent hover:bg-blue-400 hover:scale-105"
        @click.stop="emit('click:btn:start', topic)" @mouseenter="isStartBtnHovered = true"
        @mouseleave="isStartBtnHovered = false">
        <i class="mr-1 text-lg i-solar:play-circle-bold" />
        <span v-if="isStartBtnHovered">Start a quick event</span>
      </button>
      <button
        class="flex justify-center items-center p-1 bg-transparent hover:bg-pink-600 transition hover:scale-105 rounded-xl text-pink-400 hover:text-gray-100 w-8 h-7 text-lg"
        @click.stop="emit('click:btn:delete', topic)">
        <i class="i-solar:trash-bin-minimalistic-bold" />
      </button>
    </template>
  </Card>
</template>
