/* eslint-disable @typescript-eslint/consistent-type-definitions */
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { markRaw } from 'vue';
import App from './App.vue';
import router from './router';
import type { Router } from 'vue-router';
import vuetify from './plugins/vuetify';

const app = createApp(App);

// Inject router into store
declare module 'pinia' {
	export interface Pinia {router: () => Router;
	}
	export interface PiniaCustomProperties {router: Router;
	}
}

const pinia = createPinia();
pinia.use(({ store }) => {
	store.router = markRaw(router);
});
pinia.router = (): Router => router;

app.
	use(router).
	use(pinia).
	use(vuetify).
	mount('#app');
