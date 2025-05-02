import { defineStore } from 'pinia';
import { CpuMeasure, ModuleName } from '../types';

export const cpuUsageModule = defineStore(ModuleName.CpuUsage, {

	state: () => ({
		current: 0,
		average_pause: 0,
		average_resume: 0
	}),
	actions: {

		set_all (x: CpuMeasure): void {
			this.current = x.current;
			this.average_pause = x.pause ?? 0;
			this.average_resume = x.resume ?? 0;
		}
	}
});
