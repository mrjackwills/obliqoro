import { defineStore } from 'pinia';
import { ModuleName } from '../types';

export const snackModule = defineStore(ModuleName.Snack, {
	state: () => ({
		icon: '',
		loading: false,
		message: '',
		timeout: 0,
		visible: false
	}),

	actions: {
		set_icon (value: string) {
			this.icon = value;
		},
		set_loading (value: boolean) {
			this.loading = value;
		},
		set_message (value: string) {
			this.message = value;
		},
		set_timeout (value: number) {
			this.timeout = value;
		},
		set_visible (value: boolean) {
			this.visible = value;
		}
	}
});