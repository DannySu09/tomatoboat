 <script setup lang="ts">
import { ref } from "vue";
import { createTopic } from '../db';

const title = ref('');
const desc = ref('');

function handleCreateBtnClick(evt: Event) {
  evt.preventDefault();
  evt.stopPropagation();

  createTopic({
    title: title.value,
    desc: desc.value
  });
}

</script>

<template>
  <div>
    <input name="title" type="text" placeholder="title" v-model="title">
    <textarea name="desc" cols="30" rows="10" v-model="desc"></textarea>
    <button @click="handleCreateBtnClick">+ New Topic</button>
  </div>
</template>
