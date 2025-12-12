/* eslint-disable @typescript-eslint/consistent-type-definitions */
import type { Router } from 'vue-router'
import { createPinia } from 'pinia'
import { createApp, markRaw } from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import router from './router'

const app = createApp(App)

// Inject router into store
declare module 'pinia' {
	export interface Pinia { router: () => Router }
	export interface PiniaCustomProperties { router: Router }
}

const pinia = createPinia()
pinia.use(({ store }) => {
	store.router = markRaw(router)
})
pinia.router = (): Router => router

app
	.use(router)
	.use(pinia)
	.use(vuetify)
	.mount('#app')
