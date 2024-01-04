<script setup lang="ts">
import { onMounted, reactive, ref, Transition, nextTick } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import Button from '../components/base/Button.vue';
import Icon from '../components/base/Icon.vue';
import Modal from '../components/base/Modal.vue';

import { default as Timer, State as TimerState } from '../components/Timer/index.vue';

import routes from '../routes';
import * as db from '../db/index';
import type { NewWork, Topic, Work } from '../db/types';
import { invoke } from '@tauri-apps/api';
import { notify } from '../utils/notification';

const router = useRouter();
const route = useRoute();

const topicId = route.params.id as string;

const topic = ref<Topic>();
const currentWorks = ref<Work[]>([]);
const descriptionTextareaRef = ref<HTMLTextAreaElement>();

let workState = reactive<NewWork>({
  began_at: 0,
  ended_at: 0,
  desc: '',
  topic_id: topicId
});
const timerState = ref<TimerState>('stopped');
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

function showStartWithContextModal() {
  dialogState.visible = true;
  nextTick(() => {
    descriptionTextareaRef.value?.focus();
  });
}

function startWork(isContinued = false) {
  timerState.value = 'focus';
  workState.began_at = Date.now();

  if (isContinued) {
    workState.desc = workState.desc ?? topic.value?.title;
  } else {
    workState.desc = dialogState.description ?? topic.value?.title ?? '';
  }


  if (dialogState.visible) {
    dialogState.visible = false;
    dialogState.description = '';
  }

  invoke("block_websites");
}

function startBreak() {
  timerState.value = 'break';
  workState.began_at = Date.now();

  invoke("unblock_websites");
}

function giveUpWork() {
  workState.desc = '';
  workState.began_at = 0;

  dialogState.description = '';
  timerState.value = "stopped";
}

async function handleFocusEnded() {
  // when the break is ended
  if (timerState.value === "break") {
    return;
  }

  invoke("unblock_websites");
  workState.ended_at = Date.now();
  await db.createWork(workState);
  getCurrentTopicWorks();
  startBreak();
  notify("You just earn a tomato!", "");
}

onMounted(() => {
  getTopicDetail();
  getCurrentTopicWorks();

  if (route.query.autoStart === 'true') {
    startWork();
  }
});
</script>

<template>
  <div class="flex flex-col h-full">
    <header class="flex flex-grow-0 flex-shrink-0 justify-between px-2 h-8">
      <Button v-if="timerState == 'stopped'" @click="goBackHome" class-name="w-8 h-full text-3xl">
        <template #icon>
          <Icon class-name="i-solar:alt-arrow-left-outline" />
        </template>
      </Button>
      <div class="w-8" v-else />
      <div class="text-center">
        <h1 class="text-blue-500 text-lg">{{ workState.desc || topic?.title }}</h1>
        <p v-if="timerState !== 'focus' && !workState.desc && topic?.desc" class="text-sm text-pink-700">{{ topic?.desc }}</p>
      </div>
      <div class="w-8" />
    </header>

    <!-- The clock -->
    <div class="relative mt-10 flex-grow-1 flex-shrink-1 overflow-hidden">
      <Transition>
        <div v-if="timerState == 'stopped'" class="h-full">
          <div class="flex justify-center items-center">
            <div class="text-center text-xl">
              <Button class-name="box-content p-3" @click="showStartWithContextModal">
                <template #icon>
                  <Icon class-name="text-xl i-solar:add-circle-linear mr-2" />
                </template>
                <span class="text-sm">
                  Start a work with context
                </span>
              </Button>
              <Button @click="() => startWork()" class-name="box-content p-3 ml-3">
                <template #icon>
                  <Icon class-name="i-solar:play-circle-bold-duotone mr-2" />
                </template>
                <span class="text-base">Start a work!</span>
              </Button>
            </div>
          </div>

          <!-- tomato wall -->
          <div v-if="currentWorks.length > 0" class="w-1/2 lg:w-1/3 mx-auto text-center pt-25 overflow-auto flex-grow-1 opacity-80">
            <div class="inline-block w-14 h-14 p-1 mb-2 mr-2 bg-gray-200 rounded-xl" v-for="_ in currentWorks">
              <img src="../assets/tomato_icon.png" alt="tomato">
            </div>
          </div>
          <div v-else class="pt-25 text-center text-lg text-gray-400">
            You haven't grow any tomato today!
          </div>


        </div>
        <div v-else class="absolute top-0 left-0  w-full text-center">
          <div v-if="timerState === 'focus'" class="m-auto w-20 flex items-center justify-center">
            <span class="h-8 text-3xl mr-2">
              <Icon class="text-blue-500 i-solar:clock-square-bold-duotone" />
            </span>
            <span class="text-gray-300 text-sm">
              Focusing...
            </span>
          </div>
          <div v-if="timerState === 'break'" class="m-auto w-20 flex items-center justify-center">
            <span class="text-3xl mr-2">
              <Icon class="text-blue-500 i-solar:cup-hot-bold-duotone" />
            </span>
            <span class="text-gray-300 text-sm">
              Relaxing...
            </span>

          </div>
          <div class="mb-10">
            <Timer @timer:ended="handleFocusEnded" @timer:paused="giveUpWork" :current-state="timerState" />
          </div>
          <Button v-if="timerState == 'break'" state="highlight" @click="() => startWork(true)" class="box-content py-3 px-3 mr-3">
            <template #icon>
              <Icon class="mr-2 text-4xl i-solar:refresh-circle-bold-duotone" />
            </template>
            <span class="text-sm">
              Back to work
            </span>
          </Button>
          <Button state="danger" @click="giveUpWork" class-name="box-content text-3xl py-3 px-3 mx-auto">
            <template #icon>
              <Icon class-name="text-4xl i-solar:stop-circle-bold-duotone mr-2" />
            </template>
            <span v-if="timerState === 'focus'" class="text-sm">
              Stop current work
            </span>
            <span v-if="timerState === 'break'" class="text-sm">
              Stop the circle
            </span>
          </Button>
        </div>
      </Transition>
    </div>
  </div>

  <!-- start a work with context -->
  <Modal :visible="dialogState.visible" @hide="dialogState.visible = false">
    <template #title>
      Start a work with context
    </template>
    <template #body>
      <textarea ref="descriptionTextareaRef" autofocus class="w-full bg-transparent text-base text-gray-7 mt-8 p-4"
        v-model="dialogState.description" cols="30" rows="3"
        placeholder="enter the description of the work..."></textarea>
    </template>
    <template #footer>
      <Button :disabled="!dialogState.description" @click="() => startWork()" state="default" class-name="text-blue-500 w-1/2 h-8 rounded-sm">
        Ok
      </Button>
    </template>
  </Modal>
</template>