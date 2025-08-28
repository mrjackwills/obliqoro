<template>
	<v-row align='center' justify='space-between' class='ma-0 pa-0'>

		<v-col cols='5' class='ma-0 pa-0 text-primary text-left'>
			<v-row align='center' justify='start' class='ma-0 pa-0'>
				<template v-if='!paused'>
					<v-col cols='auto' class='ma-0 pa-0 mr-2'>
						<v-icon :icon='mdiCoffeeOutline' class='' />
					</v-col>
					<v-col cols='auto' class='ma-0 pa-0'>
						{{ next_in }}
					</v-col>
				</template>
			</v-row>
		</v-col>

		<v-col cols='2' class='ma-0 pa-0'>
			<v-btn @click='toggle_pause' color='primary' block rounded='sm' class='ma-0 pa-0' >
				<v-row align='center' justify='start' class='ma-0 pa-0'>
					<v-col cols='auto' class='ma-0 pa-0 mr-1'>
						<v-icon :icon='pauseIcon' class='' />
					</v-col>
					<v-col cols='auto' class='ma-0 pa-0'>
						{{ pauseText }}
					</v-col>
				</v-row>
			</v-btn>
		</v-col>

		<v-col cols='5' class='ma-0 pa-0 text-primary'>
			<v-row align='center' justify='end' class='ma-0 pa-0' v-if='!paused'>
				<v-col cols='auto' class='ma-0 pa-0'>
					{{ sessions_before_long }}
				</v-col>
				<v-col cols='auto' class='ma-0 pa-0 ml-2'>
					<v-icon :icon='mdiWeatherNight' class='' />
				</v-col>
			</v-row>

		</v-col>
	</v-row>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { InvokeMessage } from '@/types';
import { mdiCoffeeOutline, mdiPlay, mdiPause, mdiWeatherNight } from '@mdi/js';
import { snackError } from '@/services/snack';

const settingStore = settingModule();

const next_in = computed(() => nextbreakModule().nextbreak);

const sessions_before_long = computed(() =>	settingStore.session_before_next_long_break);

const paused = computed({
	get (): boolean {
		return settingStore.paused;
	},
	set (b: boolean) {
		settingStore.set_paused(b);
	}
});

const toggle_pause = async (): Promise<void> => {
	paused.value = !paused.value;
	try {
		await invoke(InvokeMessage.TogglePause);
	} catch (e) {
		snackError({ message: `Unable to pause: ${e}` });
	}
};

const pauseIcon = computed(() => paused.value ? mdiPlay : mdiPause);
const pauseText = computed(() => paused.value ? 'resume' : 'pause');
</script>
