 <script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const title = ref('');
const desc = ref('');

function createTopic(evt: Event) {
  evt.preventDefault();
  evt.stopPropagation();

  invoke('create_topic', {
    invokeMessage: JSON.stringify({
      title: title.value,
      desc: desc.value
    })
  });
}

</script>

<template>
  <div>
    <input name="title" type="text" placeholder="title" v-model="title">
    <textarea name="desc" cols="30" rows="10" v-model="desc"></textarea>
    <button @click="createTopic">+ New Topic</button>
  </div>
</template>
