<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';

type State = 'focus' | 'break';
type Events = {
  (e: 'timer:end', type: State): void;
}

const startTime = ref(0);
const currentTime = ref(0);
const currentState = ref<State>('focus');

const FOCUS_DURATION = 25 * 60 * 1000;
const BREAK_DURATION = 5 * 60 * 1000;

const duration = computed(() => {
  if (currentState.value === 'focus') {
    return FOCUS_DURATION;
  }

  if (currentState.value === 'break') {
    return BREAK_DURATION;
  }

  return 0;
});

const emit = defineEmits<Events>();

const countdownTime = computed(() => {
  const totalSeconds = Math.round((duration.value - currentTime.value + startTime.value) / 1000);

  if (totalSeconds === 0) {
    return '';
  }
  const seconds = totalSeconds % 60;
  const minutes = Math.round((totalSeconds - seconds) / 60);

  return `${minutes > 9 ? minutes : '0' + minutes} : ${seconds > 9 ? seconds : '0' + seconds}`;
});

function run() {
  if (startTime.value === 0) {
    startTime.value = Date.now();
    currentTime.value = Date.now();
  }
  window.requestAnimationFrame(() => {
    if (Date.now() - currentTime.value >= 1000) {
      if (currentTime.value - startTime.value >= duration.value) {
        emit('timer:end', currentState.value);
        return;
      }

      currentTime.value = currentTime.value + 1000;
      run();
    }
    run();
  });
}

onMounted(run)

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
    font-size: 6rem;
  }
</style>
