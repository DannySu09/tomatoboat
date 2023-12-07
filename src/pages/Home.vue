<script setup lang="ts">
import { ref, onMounted } from "vue";
import CreateTopic from "../components/CreateTopic.vue";
import TopicEntry from "../components/base/TopicEntry.vue";

import { getRecentTopics } from "../db";
import type { Topic } from "../db/types";

const recentTopics = ref<Topic[]>([]);

async function handleMounted() {
  recentTopics.value = await getRecentTopics() as Topic[];
}

onMounted(handleMounted)

</script>

<template>
  <div class="container font-sans h-full w-full bg-pink-100 p-8">
    <TopicEntry v-for="topic in recentTopics" role="button" :topic="topic" class-name="w-full" />
    <CreateTopic :on-topic-created="handleMounted">
      <i
        role="button"
        class="fixed bottom-10 right-6 i-solar:add-square-bold inline-block w-12 h-12 text-blue-400 hover:text-pink-700 hover:rotate-90 transition" />
    </CreateTopic>
  </div>
</template>

<style scoped>
.container {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-gap: 15px;
}
</style>
