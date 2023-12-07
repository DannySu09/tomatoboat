import { createApp } from "vue";
import App from "./App.vue";
import './styles.css';
import 'virtual:uno.css';

import { init } from './db';

init(() => {
  createApp(App).mount("#app");
});
