import { createApp } from "vue";
import { createRouter, createWebHistory } from 'vue-router';

import Root from './Root.vue';
import Home from './pages/Home.vue';
import Topic from './pages/Topic.vue';

import routes from './routes';

import '@unocss/reset/tailwind-compat.css';
import './styles.css';
import 'virtual:uno.css';

import { initDb } from './db';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: routes.home, component: Home },
    { name: 'topic', path: routes.topic, component: Topic },
  ]
});

const app = createApp(Root);

app.use(router);

initDb(() => {
  app.mount("#app");
});
