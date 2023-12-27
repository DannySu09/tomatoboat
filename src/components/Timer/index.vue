<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';
import { ref, computed, onUnmounted, watch, onMounted } from 'vue';

type State = 'focus' | 'break' | 'stopped';
type Events = {
  (e: 'timer:end', type: State): void;
}
type ClockEventPayload = {
  start_time: number;
  left_time: number;
  duration: number;
}

const { currentState } = defineProps<{
  currentState: State
}>();

const startTime = ref(0);
const leftTime = ref(0);
const clockEventListener = ref<() => any>(() => {});

const FOCUS_DURATION = 25 * 60 * 1000;
const BREAK_DURATION = 5 * 60 * 1000;

const duration = computed(() => {
  if (currentState === 'focus') {
    return FOCUS_DURATION;
  }

  if (currentState === 'break') {
    return BREAK_DURATION;
  }

  return 0;
});

const emit = defineEmits<Events>();

const countdownTime = computed(() => {
  const totalSeconds = Math.round(leftTime.value / 1000);

  if (totalSeconds === 0) {
    return '00 : 00';
  }
  const seconds = totalSeconds % 60;
  const minutes = Math.round((totalSeconds - seconds) / 60);

  return `${minutes > 9 ? minutes : '0' + minutes} : ${seconds > 9 ? seconds : '0' + seconds}`;
});

function reset() {
  appWindow.emit("clock:end", `${startTime.value}`);

  startTime.value = 0;
  leftTime.value = 0;
  clockEventListener.value();
}

async function run() {
  if (startTime.value === 0) {
    startTime.value = Date.now();
  }

  clockEventListener.value = await appWindow.listen<ClockEventPayload>("clock:run", (evt) => {
    const { payload } = evt;
    if (payload.start_time !== startTime.value) return;

    leftTime.value = payload.left_time;
  });

  invoke("start_clock", {
    startTime: startTime.value,
    duration: duration.value
  });
}

watch(() => currentState, () => {
  reset();

  if (currentState === "stopped") {
    return;
  }

  run();
});

onMounted(run);
onUnmounted(reset);

</script>

<template>
  <div class="text-center">
    <div class="countdown text-blue-500">
      {{ countdownTime }}
    </div>
  </div>
</template>

<style scoped>
  @import url('https://fonts.googleapis.com/css2?family=Inconsolata&display=swap');
  .countdown {
    font-family: Inconsolata, monospace;
    font-size: 7rem;
  }
</style>
