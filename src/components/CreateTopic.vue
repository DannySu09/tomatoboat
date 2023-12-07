<script setup lang="ts">
import { ref, computed } from "vue";

import Modal from './base/Modal.vue';
import { createTopic } from '../db';

type Props = {
  onTopicCreated?: () => void;
}

const { onTopicCreated } = defineProps<Props>()

const title = ref('');
const desc = ref('');
const dialogVisible = ref(false);

async function handleCreateBtnClick(evt: Event) {
  evt.preventDefault();
  evt.stopPropagation();

  await createTopic({
    title: title.value,
    desc: desc.value
  });

  hideDialog();
  onTopicCreated?.();
}

function hideDialog() {
  dialogVisible.value = false;
  title.value = '';
  desc.value = '';
}

function showDialog() {
  dialogVisible.value = true;
}

const isTitleValid = computed(() => {
  console.log(title.value);
  return title.value.length <= 3;
});

</script>

<template>
  <i
    role="button"
    @click="showDialog"
    class="fixed bottom-10 right-6 i-solar:add-square-bold inline-block w-12 h-12 text-blue-400 hover:text-pink-700 hover:rotate-90 transition"
  />

  <Modal
    :visible="dialogVisible"
    @show="showDialog"
    @hide="hideDialog"
  >
    <template #title>
      <i class="inline-block i-solar:document-add-line-duotone text-blue-500 text-2xl mr-2" />
      New Topic

    </template>
    <template #body>
      <input class="block w-3/4 m-auto mt-5 p-2 rounded-lg border border-blue-300 text-blue-600" name="title" type="text"
        placeholder="title..." autofocus v-model="title" />
      <textarea class="block w-3/4 m-auto mt-5 p-2 rounded-lg border border-blue-300 text-blue-600" name="desc" cols="30"
        rows="3" placeholder="description..." v-model="desc" />

    </template>
    <template #footer>
      <button
        class="flex justify-center items-center p-2 rounded-lg w-1/2 h-13 bg-blue-500 transition text-gray-100 hover:bg-pink-700 disabled:bg-gray-200"
        :disabled="isTitleValid" @click="handleCreateBtnClick">
        <i class="i-solar:add-circle-bold inline-block text-2xl mr-2" />
        New Topic
      </button>

    </template>
  </Modal>
</template>
