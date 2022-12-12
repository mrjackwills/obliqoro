import { createRouter, createWebHistory } from 'vue-router';
import { FrontEndRoutes } from '../types';
import Settings from '../Views/SettingsView.vue';
import Timer from '../Views/TimerView.vue';

const router = createRouter({
	history: createWebHistory('/'),
	routes: [
		{
			path: FrontEndRoutes.Settings,
			name: 'home',
			component: Settings
		},
		{
			path: FrontEndRoutes.Timer, component: Timer
		}
	],

});

export default router;