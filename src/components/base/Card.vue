<script setup lang="ts">

type Props = {
  role?: string;
  className?: string;
  titleClassName?: string;
}

const { role } = defineProps<Props>();

</script>

<template>
  <div :role="role"
    :class="`relative inline-block align-top p-4 w-1/3 h-30 transition bg-blue-700 hover:scale-105 hover:shadow-lg shadow-sm shadow-blue-200 rounded-3xl ${className}`">
    <h1 :class="`single-line-text leading-none mt-1 mb-1 text-lg color-gray-100 ${titleClassName}`">
      <slot name="title"></slot>
    </h1>
    <p class="single-line-text mt-1 mb-10 text-sm color-blue-300">
      <slot name="desc"></slot>
    </p>
    <div class="absolute flex justify-right items-center w-auto h-auto right-3 bottom-3 text-xs">
      <slot name="footer"></slot>
    </div>
  </div>
</template>