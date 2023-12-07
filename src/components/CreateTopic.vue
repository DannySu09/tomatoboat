 <script setup lang="ts">
import { ref, computed } from "vue";
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
  <div>
    <span
      @click="showDialog"
    >
      <slot></slot>
    </span>
    <div class="relative z-10" v-show="dialogVisible">
      <div class="fixed w-full h-full left-0 top-0 transition bg-gray-700 opacity-60" @click="hideDialog"/>
      <div class="fixed w-2/3 h-2/3 left-1/6 top-1/6 bg-gray-100 rounded-lg p-5">
        <button
          class="absolute flex items-center justify-center right-3 top-3 w-10 h-10 rounded-2xl transition bg-transparent text-blue-300 hover:text-blue-500 hover:bg-gray-200"
          @click="hideDialog"
        >
          <i class="inline-block i-solar:close-circle-bold text-xl" />
        </button>
        <div class="flex items-center justify-center text-blue-600">
          <i class="inline-block i-solar:document-add-line-duotone text-blue-500 text-2xl mr-2" />
          New Topic
        </div>
        <div class="py-3">
          <input class="block w-3/4 m-auto mt-5 p-2 rounded-lg border border-blue-300 text-blue-600" name="title" type="text" placeholder="title..." autofocus v-model="title" />
          <textarea class="block w-3/4 m-auto mt-5 p-2 rounded-lg border border-blue-300 text-blue-600" name="desc" cols="30" rows="3" placeholder="description..." v-model="desc" />
        </div>
        <div class="flex justify-center items-center absolute bottom-0 left-0 w-full p-3 border-t border-gray-200">
          <button
            class="flex justify-center items-center p-2 rounded-lg w-1/2 h-13 bg-blue-500 transition text-gray-100 hover:bg-pink-700 disabled:bg-gray-200"
            :disabled="isTitleValid"
            @click="handleCreateBtnClick"
          >
            <i class="i-solar:add-circle-bold inline-block text-2xl mr-2" />
            New Topic
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
