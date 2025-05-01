import { defineStore } from 'pinia';
import { ModuleName } from '../types';

export const nextbreakModule = defineStore(ModuleName.NextBreak, {

	state: () => ({ nextbreak: '' }),

	actions: {
		set_next_break (x: string): void {
			this.nextbreak = x;
		}
	}
});
