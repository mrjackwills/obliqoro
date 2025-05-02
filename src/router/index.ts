import { createRouter, createWebHistory } from 'vue-router';
import { FrontEndNames, FrontEndRoutes } from '../types';
import Settings from '../Views/SettingsView.vue';
import Timer from '../Views/TimerView.vue';

const router = createRouter({
	history: createWebHistory('/'),
	routes: [
		{
			path: FrontEndRoutes.Settings,
			name: FrontEndNames.Settings,
			component: Settings
		},
		{
			path: FrontEndRoutes.Timer,
			name: FrontEndNames.Timer,
			component: Timer
		}
	]

});

export default router;