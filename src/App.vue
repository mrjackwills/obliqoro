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
import { invoke } from '@tauri-apps/api/core';
import { ListenMessage, ShowTimer, FrontEndRoutes, FrontEndState, BuildInfo, InvokeMessage, CpuMeasure } from '@/types';
import { useRouter } from 'vue-router';
import { snackError } from '@/services/snack';
// import { getCurrentWindow } from '@tauri-apps/api/window';

const router = useRouter();
const route = useRoute();
const intervalStore = intervalModule();
const settingStore = settingModule();
const cpuUsageStore = cpuUsageModule();
const packageinfoStore = packageinfoModule();

const on_settings = computed(() => route.fullPath === FrontEndRoutes.Settings);

/*
 * Disable webview hotkeys, reloading, searching, etc
 * also disable context menu
 */
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

	await listen(ListenMessage.GoToTimer, (event: Event<ShowTimer>) => {
		router.push(FrontEndRoutes.Timer);
		intervalStore.set_interval(event.payload.interval);
		intervalStore.set_original(event.payload.interval);
		intervalStore.set_strategy(event.payload.strategy);
	});

	await listen(ListenMessage.Cpu, async (event: Event<CpuMeasure>) => cpuUsageStore.set_all(event.payload));
	await listen(ListenMessage.Error, async (event: Event<string>) => snackError({ message: event.payload }));
	await listen(ListenMessage.GetSettings, async (event: Event<FrontEndState>) => settingStore.set_current_state(event.payload));
	await listen(ListenMessage.GoToSettings, () => router.push(FrontEndRoutes.Settings));
	await listen(ListenMessage.NextBreak, async (event: Event<string>) => nextbreakModule().set_next_break(event.payload));
	await listen(ListenMessage.NumberSessionsBeforeLong, async (event: Event<string>) => settingStore.set_session_before_next_long_break(event.payload));
	await listen(ListenMessage.OnBreak, async (event: Event<number>) => intervalStore.set_interval(event.payload));
	await listen(ListenMessage.PackageInfo, async (event: Event<BuildInfo>) => packageinfoStore.set_all(event.payload));
	await listen(ListenMessage.Paused, async (event: Event<boolean>) => settingStore.set_paused(event.payload));

	/*
	 * await listen(ListenMessage.Fullscreen, async (event: Event<boolean>) => {
	 * 	await getCurrentWindow().setFullscreen(event.payload);
	 * });
	 */


	await invoke(InvokeMessage.Init);
});

</script>

<style>
#obliqoro {
	overflow: hidden !important;
}
</style>
