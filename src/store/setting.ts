/* eslint-disable space-before-function-paren */
import { defineStore } from 'pinia';
import { ModuleName } from '../types';

export const settingModule = defineStore(ModuleName.Setting, {

	state: () => ({
		autostart: false,
		fullscreen: false,
		long_break_as_sec: 0,
		number_session_before_break: 0,
		paused: false,
		session_as_sec: 0,
		session_before_next_long_break: '',
		short_break_as_sec: 0,
	}),
	actions: {

		set_autostart(x: boolean): void {
			this.autostart = x;
		},

		set_fullscreen(x: boolean): void {
			this.fullscreen = x;
		},
		set_session_as_sec(x: number): void {
			this.session_as_sec = x;
		},
		set_short_break_as_sec(x: number): void {
			this.short_break_as_sec = x;
		},
		set_long_break_as_sec(x: number): void {
			this.long_break_as_sec = x;
		},
		set_number_session_before_break(x: number): void {
			this.number_session_before_break = x;
		},
		set_paused(x: boolean): void {
			this.paused = x;
		},
		set_session_before_next_long_break(x: string): void {
			this.session_before_next_long_break = x;
		},
	},
});
