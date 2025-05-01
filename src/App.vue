<template>
	<v-app class='ma-0 pa-0 fill-height unselectable' id='obliqoro'>
		<v-main>
			<AppBar v-if='on_settings' />
			<router-view />
			<SnackBar />
			<TheFooter v-if='on_settings' />
		</v-main>
	</v-app>
</template>

<script setup lang="ts">
import { listen, Event } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { ListenMessage, ShowTimer, FrontEndRoutes, CurrentState, InvokeMessage, PackageInfo, CpuMeasure } from './types';
import { useRouter } from 'vue-router';
import { snackError } from './services/snack';

const router = useRouter();
const route = useRoute();
const intervalStore = intervalModule();
const settingStore = settingModule();
const cpuUsageStore = cpuUsageModule();
const packageinfoStore = packageinfoModule();

const on_settings = computed(() => route.fullPath === FrontEndRoutes.Settings);

/// Disable webview hotkeys, reloading, searching, etc
/// Also disable context menu
const disable_hotkeys = (): void => {
	window.addEventListener('contextmenu', (event) => event.preventDefault());

	window.addEventListener('keydown', (event) => {
		const key = event.key.toLowerCase();
		const f_keys = ['f1', 'f3', 'f7'];
		if (f_keys.includes(key) || (event.ctrlKey || event.metaKey && key === 'f')) event.preventDefault();
	}, true);
};


onBeforeMount(async () => {
	disable_hotkeys();

	await listen(ListenMessage.NumberSessionsBeforeLong, async (event: Event<string>) => {
		settingStore.set_session_before_next_long_break(event.payload);
	});

	await listen(ListenMessage.Paused, async (event: Event<boolean>) => {
		settingStore.set_paused(event.payload);
	});

	await listen(ListenMessage.Cpu, async (event: Event<CpuMeasure>) => {
		cpuUsageStore.set_all(event.payload);
	});

	await listen(ListenMessage.GoToSettings, () => {
		router.push(FrontEndRoutes.Settings);
	});

	await listen(ListenMessage.GoToTimer, (event: Event<ShowTimer>) => {
		router.push(FrontEndRoutes.Timer);
		intervalStore.set_interval(event.payload.interval);
		intervalStore.set_original(event.payload.interval);
		intervalStore.set_strategy(event.payload.strategy);
	});

	await listen(ListenMessage.OnBreak, async (event: Event<number>) => {
		intervalStore.set_interval(event.payload);
	});

	await listen(ListenMessage.Error, async (event: Event<string>) => {
		snackError({ message: event.payload });
	});

	await listen(ListenMessage.PackageInfo, async (event: Event<PackageInfo>) => {
		packageinfoStore.set_homepage(event.payload.homepage);
		packageinfoStore.set_version(event.payload.version);
		packageinfoStore.set_build_date(event.payload.build_date);
	});

	await listen(ListenMessage.GetSettings, async (event: Event<CurrentState>) => {
		settingStore.set_current_state(event.payload);
	});

	await listen(ListenMessage.NextBreak, async (event: Event<string>) => {
		nextbreakModule().set_next_break(event.payload);
	});

	await invoke(InvokeMessage.Init);

});

</script>

<style>
#obliqoro {
	overflow: hidden !important;
}
</style>
