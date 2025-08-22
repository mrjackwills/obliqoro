import { defineStore } from 'pinia';
import { FrontEndState, ModuleName } from '../types';

export const settingModule = defineStore(ModuleName.Setting, {

	state: () => ({
		start_on_boot: false,
		fullscreen: false,
		long_break_as_sec: 0,
		number_session_before_break: 0,
		paused: false,
		session_as_sec: 0,
		session_before_next_long_break: '',
		short_break_as_sec: 0,
		auto_pause: false,
		auto_resume: false,
		auto_pause_threshold: 0,
		auto_resume_threshold: 0,
		auto_pause_timespan_sec: 0,
		auto_resume_timespan_sec: 0
	}),

	getters: {
		get_current_state (): FrontEndState {
			return {
				auto_pause_threshold: this.auto_pause_threshold,
				auto_pause_timespan_sec: this.auto_pause_timespan_sec,
				auto_pause: this.auto_pause,
				auto_resume_threshold: this.auto_resume_threshold,
				auto_resume_timespan_sec: this.auto_resume_timespan_sec,
				auto_resume: this.auto_resume,
				fullscreen: this.fullscreen,
				long_break_as_sec: this.long_break_as_sec,
				number_session_before_break: this.number_session_before_break,
				paused: this.paused,
				session_as_sec: this.session_as_sec,
				short_break_as_sec: this.short_break_as_sec,
				start_on_boot: this.start_on_boot
			};
		}
	},
	actions: {

		set_start_on_boot (x: boolean): void {
			this.start_on_boot = x;
		},

		set_auto_pause (x: boolean): void {
			this.auto_pause = x;
		},

		set_auto_pause_threshold (x: number): void {
			this.auto_pause_threshold = x;
		},

		set_auto_pause_timespan_sec (x: number): void {
			this.auto_pause_timespan_sec = x;
		},

		set_auto_resume (x: boolean): void {
			this.auto_resume = x;
		},

		set_auto_resume_threshold (x: number): void {
			this.auto_resume_threshold = x;
		},

		set_auto_resume_timespan_sec (x: number): void {
			this.auto_resume_timespan_sec = x;
		},

		set_fullscreen (x: boolean): void {
			this.fullscreen = x;
		},
		set_session_as_sec (x: number): void {
			this.session_as_sec = x;
		},
		set_short_break_as_sec (x: number): void {
			this.short_break_as_sec = x;
		},
		set_long_break_as_sec (x: number): void {
			this.long_break_as_sec = x;
		},
		set_number_session_before_break (x: number): void {
			this.number_session_before_break = x;
		},
		set_paused (x: boolean): void {
			this.paused = x;
		},
		set_session_before_next_long_break (x: string): void {
			this.session_before_next_long_break = x;
		},
		set_current_state (x: FrontEndState): void {
			this.start_on_boot = x.start_on_boot;
			this.fullscreen = x.fullscreen;
			this.long_break_as_sec = x.long_break_as_sec;
			this.number_session_before_break = x.number_session_before_break;
			this.paused = x.paused;
			this.session_as_sec = x.session_as_sec;
			this.short_break_as_sec = x.short_break_as_sec;
			this.auto_pause = x.auto_pause;
			this.auto_resume = x.auto_resume;
			this.auto_pause_threshold = x.auto_pause_threshold;
			this.auto_resume_threshold = x.auto_resume_threshold;
			this.auto_pause_timespan_sec = x.auto_pause_timespan_sec;
			this.auto_resume_timespan_sec = x.auto_resume_timespan_sec;
		}
	}
});
