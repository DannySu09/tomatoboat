<script setup lang="ts">
import { onMounted, reactive, ref, Transition } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { format } from 'date-fns';

import Button from '../components/base/Button.vue';
import Icon from '../components/base/Icon.vue';
import Modal from '../components/base/Modal.vue';

import { default as Timer, State as TimerState } from '../components/Timer/index.vue';

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

function startWork() {
  timerState.value = 'focus';
  workState.began_at = Date.now();
  workState.desc = dialogState.description ?? topic.value?.title ?? '';

  if (dialogState.visible) {
    dialogState.visible = false;
    dialogState.description = '';
  }
}

function giveUpWork() {
  for (let p in initialWorkState) {
    // @ts-ignore
    workState[p as keyof NewWork] = initialWorkState[p as keyof NewWork]
  }

  dialogState.description = '';
  timerState.value = "stopped";
}

async function handleTimerStatusChanged(state: TimerState) {
  switch(state) {
    case "break":
      timerState.value = "break";
      await db.createWork(workState);
      getCurrentTopicWorks();
      break;
    case "stopped":
      giveUpWork();
      break;
    default:
  }
}

function formatTime(ts?: number) {
  if (!ts) {
    return '';
  }
  return format(ts, 'mm-dd HH:MM')
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
  <div class="flex flex-col">
    <header class="flex flex-grow-0 flex-shrink-0 justify-between mb-10 px-2 h-8">
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
      <div class="w-8" />
    </header>

    <!-- The clock -->
    <div class="relative mt-20 flex-grow-1 flex-shrink-1">
      <Transition>
        <div v-if="timerState == 'stopped'" class="flex">
          <div class="flex-grow-1 flex justify-center items-center">
            <div class="text-center">
              <Button @click="startWork" class-name="box-content text-3xl p-3 mb-3">
                <template #icon>
                  <Icon class-name="i-solar:play-circle-bold-duotone mr-2" />
                </template>
                <span class="text-base">Start working!</span>
              </Button>
              <br>
              <Button class-name="text-sm p-3" @click="dialogState.visible = true">
                <template #icon>
                  <Icon class-name="text-xl i-solar:add-circle-linear mr-2" />
                </template>
                <span class="text-xs">
                  Start a work with context
                </span>
              </Button>

            </div>
          </div>
          <div v-if="currentWorks.length > 0" class="flex-grow-1 overflow-auto opacity-70">
            <div class="p-2 mb-2 bg-gray-200 text-center rounded-xl" v-for="work in currentWorks">
              <p class="text-sm text-blue-600">
                {{ work.desc || topic?.desc }}
              </p>
              <small class="text-xs text-blue-300">{{ formatTime(work.began_at) }} to {{ formatTime(work.ended_at) }}</small>
            </div>
          </div>
        </div>
        <div v-else class="absolute top-0 left-0  w-full text-center">
          <div class="mb-10">
            <Timer @timer:changed="handleTimerStatusChanged" :current-state="timerState" />
          </div>
          <Button state="danger" @click="giveUpWork" class-name="box-content text-3xl py-3 px-3 mx-auto">
            <template #icon>
              <Icon class-name="text-4xl i-solar:stop-circle-bold-duotone mr-2" />
            </template>
            <span v-if="timerState === 'focus'" class="text-sm">
              Stop current work
            </span>
            <span v-if="timerState === 'break'">
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