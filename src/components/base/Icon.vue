<script setup lang="ts">
type Props = {
  className?: string;
}

defineProps<Props>();
</script>

<template>
  <i :class="`inline-block ${className}`" />
</template>