<script setup lang="ts">
import { onMounted, ref } from 'vue';
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
const events = ref<Event[]>();

const dialogVisible = ref(false);

function goBackHome() {
  router.replace(routes.home)
}

async function getTopicDetail () {
  const res = await db.getTopic(topicId) as Topic[];
  topic.value = res[0];
}

async function getEvents() {
  const res = await db.getEvents(topicId) as Event[];
  events.value = res;
}

onMounted(() => {
  getTopicDetail();
  getEvents();
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
    <div class="flex items-center mt-2 ml-6 mr-6">
      <Button class-name="box-content text-3xl p-2 mr-5">
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
    <div>
      <Timer />
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