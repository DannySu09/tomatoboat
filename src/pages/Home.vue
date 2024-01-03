<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";

import Icon from '../components/base/Icon.vue';
import CreateTopic from "../components/CreateTopic.vue";
import TopicEntry from "../components/TopicEntry/index.vue";

import { getRecentTopics, deleteTopic } from "../db";

import type { Topic } from "../db/types";

const router = useRouter();
const recentTopics = ref<Topic[]>([]);

async function getTopics() {
  recentTopics.value = await getRecentTopics() as Topic[];
}

function goToTopic(topic: Topic, autoStart = false) {
  router.push({
    name: 'topic',
    params: {
      id: topic.id
    },
    query: {
      autoStart: autoStart.toString()
    }
  });
}

async function handleDeletingTopic(topic: Topic) {
  await deleteTopic(topic.id);
  getTopics();
}

onMounted(getTopics)

</script>

<template>
  <div v-if="recentTopics.length === 0" class="w-full mt-20 text-center text-blue-300">
    <div>
      <Icon class="block mb-2 i-solar:notification-remove-bold text-6xl text-blue-300" />
    </div>
    Not any topic here, please click the button at the <b class="text-blue-500">right bottom corner</b> to add.
  </div>
  <div class="container">
    <TopicEntry
      v-for="topic in recentTopics"
      :topic="topic"
      role="button"
      @click:card="goToTopic"
      @click:btn:start="goToTopic(topic, true)"
      @click:btn:delete="handleDeletingTopic"
      class-name="w-full" />
  </div>

  <CreateTopic :on-topic-created="getTopics">
    <i
      role="button"
      class="fixed block bottom-10 right-6 i-solar:add-square-bold inline-block w-12 h-12 text-blue-400 hover:text-pink-700 hover:scale-105 transition shadow-blue-200 shadow-lg hover:shadow-pink-400" />
  </CreateTopic>
</template>

<style scoped>
.container {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-gap: 15px;
}
</style>
