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

  if(dialogState.visible) {
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
  if (timerState.value === "break") {
    timerState.value = "focus";
    return;
  }

  timerState.value = "break";
  // the work hasn't been started yet
  if (!workState.began_at) {
    return;
  }

  await db.createWork(workState);

  getCurrentTopicWorks();
  giveUpWork();

  invoke("unblock_websites");
}

onMounted(() => {
  getTopicDetail();
  getCurrentTopicWorks();
});
</script>

<template>
  <div>
    <header class="flex justify-between px-2">
      <Button v-if="timerState == 'stopped'" @click="goBackHome" class-name="w-8 h-8 text-3xl">
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
        <div v-if="timerState == 'stopped'" class="absolute top-10 left-0 w-full mt-2 ml-6 mr-6 flex items-center justify-center">
          <Button @click="startWork" class-name="box-content text-3xl p-2 mr-5">
            <template #icon>
              <Icon class-name="i-solar:play-circle-bold-duotone mr-2" />
            </template>
            <span class="text-xs">Start Now!</span>
          </Button>
          <Button class-name="text-lg px-2" @click="dialogState.visible = true">
            <template #icon>
              <Icon class-name="i-solar:add-circle-linear mr-2" />
            </template>
            <span class="text-xs">
              Start a work with context
            </span>
          </Button>
        </div>
        <div v-else class="absolute top-0 left-0 w-full text-center">
          <div class="mb-20">
            <Timer @timer:end="finishWork" :current-state="timerState" />
          </div>
          <Button
            state="danger"
            @click="giveUpWork"
            class-name="box-content text-3xl py-3 px-3 mx-auto"
          >
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
  </div>

  <Modal :visible="dialogState.visible" @hide="dialogState.visible = false">
    <template #title>
      Start a work with context
    </template>
    <template #body>
      <textarea class="w-full bg-transparent text-base text-gray-7 mt-8 p-4" v-model="dialogState.description" cols="30" rows="3" placeholder="enter the description of the work..."></textarea>
    </template>
    <template #footer>
      <Button @click="startWork" state="default" class-name="text-blue-500 w-1/2 h-8 rounded-sm">
        Ok
      </Button>
    </template>
  </Modal>
</template>