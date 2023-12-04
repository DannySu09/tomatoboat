<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

type Topic = {
  id: number;
  title: string;
  desc: string;
  created_at: number;
  modified_at: number;
}

const recentTopics = ref<Topic[]>([]);

async function loadRecentTopics() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const recentTopicsJson = await invoke("get_topics") as string;
  console.log(recentTopicsJson);
  recentTopics.value = JSON.parse(recentTopicsJson);
}

onMounted(loadRecentTopics)
</script>

<template>
  <ul>
    <li v-for="topic in recentTopics">
      <p>{{ topic.title }}</p>
      <p>
        <small>{{ topic.desc }}</small>
      </p>
    </li>
  </ul>
</template>
