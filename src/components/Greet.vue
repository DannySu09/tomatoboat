<script setup lang="ts">
import { ref, onMounted } from "vue";
import { getRecentTopics } from '../db';
import type { Topic } from "../db/types";

const recentTopics = ref<Topic[]>([]);

async function handleMounted () {
  recentTopics.value = await getRecentTopics() as Topic[];
}

onMounted(handleMounted)
</script>

<template>
  <div>
    <div v-for="topic in recentTopics" class="inline-block w-56 bg-pink-600 rounded-md px-3 py-1">
      <h1 class="mt-1 mb-1 text-base color-blue-700">{{ topic.title }}</h1>
      <p class="mt-1 mb-0 text-sm color-blue-600">
        <small>{{ topic.desc }}</small>
      </p>
    </div>
  </div>
</template>
