import { defineStore } from 'pinia';
import { ModuleName, BuildInfo } from '../types';

export const packageinfoModule = defineStore(ModuleName.PackageInfo, {

	state: () => ({
		homepage: '',
		version: '',
		build_date: '',
		github_version: ''
	}),

	actions: {

		set_all (x: BuildInfo): void {
			this.build_date = x.build_date;
			this.homepage = x.homepage;
			this.version = x.version;
			this.github_version = x.github_version ?? '';
		}
	}
});
