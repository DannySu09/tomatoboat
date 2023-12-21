<script setup lang="ts">
import { onMounted, reactive, ref, Transition } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import Button from '../components/base/Button.vue';
import Icon from '../components/base/Icon.vue';
import Modal from '../components/base/Modal.vue';

import Timer from '../components/Timer/index.vue';

import routes from '../routes';
import * as db from '../db/index';
import type { Topic } from '../db/types';

const router = useRouter();
const route = useRoute();

const topicId = route.params.id as string;

const topic = ref<Topic>();

const workState = reactive({
  started: false,
  title: '',
  rounds: 0
});
const timerState = ref<'focus' | 'break'>('focus');

const dialogVisible = ref(false);

function goBackHome() {
  router.replace(routes.home)
}

async function getTopicDetail() {
  const res = await db.getTopic(topicId) as Topic[];
  topic.value = res[0];
}

function startWork(title = topic.value?.title) {
  workState.started = true;
  workState.title = title ?? '';
}

function stopWork() {
  workState.started = false;
  workState.title = '';
}

onMounted(() => {
  getTopicDetail();
});
</script>

<template>
  <div>
    <header class="flex justify-between px-2">
      <Button @click="goBackHome" class-name="w-8 h-8 text-3xl">
        <template #icon>
          <Icon class-name="i-solar:alt-arrow-left-outline" />
        </template>
      </Button>
      <div class="text-center">
        <h1 class="text-blue-500 text-lg">{{ topic?.title }}</h1>
        <p class="text-sm text-pink-700" v-if="topic?.desc">{{ topic?.desc }}</p>
      </div>
      <div />
    </header>
    <div class="relative">
      <Transition>
        <div v-if="!workState.started" class="absolute top-0 left-0 w-full flex items-center mt-2 ml-6 mr-6">
          <Button @click="startWork" class-name="box-content text-3xl p-2 mr-5">
            <template #icon>
              <Icon class-name="i-solar:play-circle-bold-duotone mr-2" />
            </template>
            <span class="text-xs">Start Now!</span>
          </Button>
          <Button class-name="text-lg px-2" @click="dialogVisible = true">
            <template #icon>
              <Icon class-name="i-solar:add-circle-linear mr-2" />
            </template>
            <span class="text-xs">
              Start a work with description
            </span>
          </Button>
        </div>
        <div v-else class="absolute top-0 left-0 w-full ">
          <Timer :current-state="timerState" />
          <Button @click="stopWork" state="danger" class-name="box-content text-3xl py-3 px-3 m-auto">
            <template #icon>
              <Icon class-name="text-5xl i-solar:stop-circle-bold-duotone mr-2" />
            </template>
            <span class="text-sm">
              Stop current work
            </span>
          </Button>
        </div>
      </Transition>
    </div>
  </div>
  <Modal :visible="dialogVisible" @hide="dialogVisible = false">
    <template #title>
      Add New Event
    </template>
    <template #body>
      Hi?
    </template>
    <template #footer>
      <Button state="default" class-name="text-blue-500 w-1/2 h-8 rounded-sm">
        Ok
      </Button>
    </template>
  </Modal>
</template>