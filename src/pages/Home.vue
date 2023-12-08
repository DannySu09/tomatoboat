<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";

import CreateTopic from "../components/CreateTopic.vue";
import TopicEntry from "../components/TopicEntry/index.vue";

import { getRecentTopics } from "../db";

import type { Topic } from "../db/types";

const router = useRouter();
const recentTopics = ref<Topic[]>([]);

async function handleMounted() {
  recentTopics.value = await getRecentTopics() as Topic[];
}

function goToTopic(topic: Topic) {
  router.push({
    name: 'topic',
    params: {
      id: topic.id
    }
  });
}

onMounted(handleMounted)

</script>

<template>
  <div class="container">
    <TopicEntry
      v-for="topic in recentTopics"
      :topic="topic"
      role="button"
      @click:card="goToTopic"
      class-name="w-full" />
  </div>
  <CreateTopic :on-topic-created="handleMounted">
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
