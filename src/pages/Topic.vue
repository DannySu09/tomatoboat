<script setup lang="ts">
import { onMounted, reactive, ref, Transition } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api';

import Button from '../components/base/Button.vue';
import Icon from '../components/base/Icon.vue';
import Modal from '../components/base/Modal.vue';

import Timer from '../components/Timer/index.vue';

import routes from '../routes';
import * as db from '../db/index';
import type { NewWork, Topic, Work } from '../db/types';

const router = useRouter();
const route = useRoute();

const topicId = route.params.id as string;
const initialWorkState: NewWork = {
  began_at: 0,
  ended_at: 0,
  desc: '',
  topic_id: topicId
};

const topic = ref<Topic>();
const currentWorks = ref<Work[]>([]);

let workState = reactive<NewWork>(initialWorkState);
const timerState = ref<'focus' | 'break' | 'stopped'>('stopped');

const dialogState = reactive({
  visible: false,
  description: ''
});

function goBackHome() {
  router.replace(routes.home)
}

async function getTopicDetail() {
  const res = await db.getTopic(topicId) as Topic[];
  topic.value = res[0];
}

async function getCurrentTopicWorks() {
  const res = await db.getWorks(topicId) as Work[];
  currentWorks.value = res;
}

function startWork() {
  timerState.value = 'focus';
  workState.began_at = Date.now();
  workState.desc = dialogState.description ?? topic.value?.title ?? '';

  if (dialogState.visible) {
    dialogState.visible = false;
    dialogState.description = '';
  }

  invoke("block_websites");
}

function giveUpWork() {
  for (let p in initialWorkState) {
    // @ts-ignore
    workState[p as keyof NewWork] = initialWorkState[p as keyof NewWork]
  }

  dialogState.description = '';
  timerState.value = "stopped";
}

async function finishWork() {
  // the work hasn't been started yet
  if (!workState.began_at) {
    return;
  }

  timerState.value = "break";
  await db.createWork(workState);

  getCurrentTopicWorks();
  invoke("unblock_websites");
}

onMounted(() => {
  getTopicDetail();
  getCurrentTopicWorks();
});
</script>

<template>
  <header class="flex justify-between mb-10 px-2 h-8">
    <Button v-if="timerState == 'stopped'" @click="goBackHome" class-name="w-8 h-full text-3xl">
      <template #icon>
        <Icon class-name="i-solar:alt-arrow-left-outline" />
      </template>
    </Button>
    <div class="w-8" v-else />
    <div class="text-center">
      <h1 class="text-blue-500 text-lg">{{ topic?.title }}</h1>
      <p class="text-sm text-pink-700" v-if="topic?.desc">{{ topic?.desc }}</p>
    </div>
    <div />
  </header>

  <!-- The clock -->
  <div class="relative">
    <Transition>
      <div v-if="timerState == 'stopped'" class="flex">
        <div class="grow-1 flex items-center justify-center">
          <div>
            <Button @click="startWork" class-name="box-content text-5xl p-5 mr-5">
              <template #icon>
                <Icon class-name="i-solar:play-circle-bold-duotone mr-2" />
              </template>
              <span class="text-lg">Start working!</span>
            </Button>
          </div>
          <div>
            <Button class-name="text-sm px-2" @click="dialogState.visible = true">
              <template #icon>
                <Icon class-name="i-solar:add-circle-linear mr-2" />
              </template>
              <span class="text-xs">
                Start a work with context
              </span>
            </Button>
          </div>
        </div>
        <div class="max-h-md grow-1 overflow-auto opacity-70">
          <div class="p-2 mb-2 bg-gray-200 text-center rounded-xl" v-for="work in currentWorks">
            <p class="text-sm text-blue-600">
              {{ work.desc || topic?.desc }}
            </p>
            <small class="text-xs text-blue-300">{{ work.began_at }}</small>
          </div>
        </div>
      </div>
      <div v-else class="absolute top-0 left-0  w-full text-center">
        <div class="mb-10">
          <Timer @timer:end="finishWork" :current-state="timerState" />
        </div>
        <Button state="danger" @click="giveUpWork" class-name="box-content text-3xl py-3 px-3 mx-auto">
          <template #icon>
            <Icon class-name="text-4xl i-solar:stop-circle-bold-duotone mr-2" />
          </template>
          <span class="text-sm">
            Stop current work
          </span>
        </Button>
      </div>
    </Transition>
  </div>

  <!-- start a work with context -->
  <Modal :visible="dialogState.visible" @hide="dialogState.visible = false">
    <template #title>
      Start a work with context
    </template>
    <template #body>
      <textarea class="w-full bg-transparent text-base text-gray-7 mt-8 p-4" v-model="dialogState.description" cols="30"
        rows="3" placeholder="enter the description of the work..."></textarea>
    </template>
    <template #footer>
      <Button @click="startWork" state="default" class-name="text-blue-500 w-1/2 h-8 rounded-sm">
        Ok
      </Button>
    </template>
  </Modal>
</template>