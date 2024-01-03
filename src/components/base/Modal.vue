<script setup lang="ts">
import { Transition } from 'vue';

type Props = {
  visible?: boolean
}

type Event = {
  (e: 'show'): void;
  (e: 'hide'): void;
}

const { visible } = defineProps<Props>();
const emits = defineEmits<Event>();

</script>

<template>
  <div tabindex="-1" class="relative z-10" v-show="visible">
    <Transition name="fade">
      <div class="fixed w-full h-full left-0 top-0 transition bg-gray-700 opacity-60" @click="emits('hide')" />
    </Transition>
    <Transition name="fade">
      <div class="fixed w-2/3 h-2/3 left-1/6 top-1/6 bg-gray-100 rounded-lg p-5">
        <button
          class="absolute flex items-center justify-center right-3 top-3 w-10 h-10 rounded-2xl transition bg-transparent text-blue-300 hover:text-blue-500 hover:bg-gray-200"
          @click="emits('hide')">
          <i class="inline-block i-solar:close-circle-bold text-xl" />
        </button>
        <div class="flex items-center justify-center text-blue-600">
          <slot name="title"></slot>
        </div>
        <div class="py-3">
          <slot name="body"></slot>
        </div>
        <div class="flex justify-center items-center absolute bottom-0 left-0 w-full p-3 border-t border-gray-200">
          <slot name="footer"></slot>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped></style>
