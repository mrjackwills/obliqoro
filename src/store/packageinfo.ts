import { defineStore } from 'pinia';
import { ModuleName } from '../types';

export const packageinfoModule = defineStore(ModuleName.PackageInfo, {

	state: () => ({
		homepage: '',
		version: '',
		build_date: ''
	}),

	actions: {
		set_build_date (x: string): void {
			this.build_date = x;
		},
		set_homepage (x: string): void {
			this.homepage = x;
		},
		set_version (x: string): void {
			this.version = x;
		}
	}
});
