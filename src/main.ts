import { createApp } from "vue";
import { createRouter, createMemoryHistory } from 'vue-router';

import Root from './Root.vue';
import Home from './pages/Home.vue';
import Topic from './pages/Topic.vue';

import '@unocss/reset/tailwind-compat.css';
import './styles.css';
import 'virtual:uno.css';

import { initDb } from './db';

const routes = [
  { path: '/', component: Home },
  { path: '/:topicId', component: Topic },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes
});

const app = createApp(Root);

app.use(router);

initDb(() => {
  app.mount("#app");
});
