import { createRouter, createWebHistory } from 'vue-router';
import { FrontEndRoutes } from '../types';
// import BlankViewVue from '../Views/BlankView.vue';
import Settings from '../Views/SettingsView.vue';
import Timer from '../Views/TimerView.vue';

const router = createRouter({
	history: createWebHistory('/'),
	routes: [
		{
			path: FrontEndRoutes.Settings,
			name: 'settings',
			component: Settings
		},
		// {
		// 	path: FrontEndRoutes.Home,
		// 	name: 'blank',
		// 	component: BlankViewVue
		// },
		{
			path: FrontEndRoutes.Timer,
			name: 'timer',
			component: Timer
		}
	],

});

export default router;