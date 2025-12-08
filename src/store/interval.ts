import { defineStore } from 'pinia'
import { BreakTypes, ModuleName } from '../types'

export const intervalModule = defineStore(ModuleName.Interval, {

	state: () => ({
		interval: 0,
		original_interval: 0,
		// TODO typeguard
		break_type: BreakTypes.Short as typeof BreakTypes,
		strategy: '',
	}),
	actions: {

		decrement () {
			this.interval--
		},
		set_interval (x: number): void {
			this.interval = x
		},
		set_strategy (x: string): void {
			this.strategy = x
		},
		set_original (x: number): void {
			this.original_interval = x
		},
		set_break_type (x: string): void {
			// TODO typeguard
			this.break_type = x === 'short' ? BreakTypes.Short : BreakTypes.Long
		},
	},
})
