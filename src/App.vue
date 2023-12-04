<script setup lang="ts">
import { WebviewWindow, getAll } from '@tauri-apps/api/window';
import Greet from "./components/Greet.vue";

function openWindow() {
  const test = new WebviewWindow('testwindow', {
    url: 'https://baidu.com'
  });

  test.show();

  console.log(getAll());
}
</script>

<template>
  <div class="container">
    <Greet />
    <button @click="openWindow">open window</button>
  </div>
</template>
