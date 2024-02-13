<template>
	<v-row class='ma-0 pa-0 no-gutters fill-height' align='center'>

		<v-row align='center' justify='center' class='ma-0 pa-0'>
			<v-col cols='9' class='ma-0 pa-0'>

				<!-- TITLE -->
				<v-row align='center' justify='center' class='ma-0 pa-0'>
					<v-col cols='auto' class='text-h4 ma-0 pa-0 text-primary'>
						Settings
					</v-col>
				</v-row>

				<hr class='my-4 hr' />

				<!-- BREAK/PAUSE INFO -->
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
						<v-btn @click='toggle_pause'  color='primary' block rounded='sm' class='ma-0 pa-0'>
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
						
				<hr class='my-4 hr' />

				<!-- SWITCHES -->
				<v-form v-on:submit.prevent class='mt-4'>
					<v-row class='ma-0 pa-0' justify='space-between'>

						<v-col v-for='(item, index) in switches' :key='index' cols='auto' class='ma-0 pa-0'>
							<v-switch
								v-model='item.model.value'
								:class='item.model.value ? "text-primary" : "text-offwhite"'
								:label='item.label'
								flat
								color='primary'
							/>
						
						</v-col>

					</v-row>
					
					<!-- SLIDERS -->
					<section v-for='(item, index) in sliders' :key='index'>

						<v-row class='text-primary ma-0 pa-0'>
							<v-col cols='auto' class='ma-0 pa-0'>
								{{ item.name }}
							</v-col>
							<v-spacer />
							<v-col cols='auto' class='ma-0 pa-0'>
								{{ item.label_value }}
							</v-col>
						</v-row>

						<v-row class='ma-0 pa-0'>
							<v-col cols='12' class='ma-0 pa-0'>
								<v-slider v-model='item.model.value' color='primary' :disabled='paused' :min='item.min'
									density='compact' :max='item.max' :step='item.step' rounded>
								</v-slider>
								<ResumeTooltip :paused='paused' />
							</v-col>
						</v-row>
					</section>

				</v-form>

				<!-- RESET BUTTON -->
				<v-row class='ma-0 pa-0 mt-6' justify='center'>
					<v-col cols='auto' class='ma-0 pa-0'>
						<v-btn @click='reset_settings' :disabled='paused' :variant='paused? "outlined" : undefined' color='primary' block rounded='sm'>
							<v-icon :icon='mdiCogRefresh' class='mr-1' />
							reset settings
						</v-btn>
					</v-col>
				
				</v-row>

			</v-col>
		</v-row>

	</v-row>
</template>
<script setup lang="ts">
import { sec_to_minutes, sec_to_minutes_only } from '../vanillaTS/second';
import { invoke } from '@tauri-apps/api/tauri';
import { InvokeMessage } from '../types';
import { snackError } from '../services/snack';
import { mdiCogRefresh, mdiCoffeeOutline, mdiPlay, mdiPause, mdiWeatherNight } from '@mdi/js';
const settingStore = settingModule();

const next_in = computed(() => nextbreakModule().nextbreak);

const sessions_before_long = computed(() =>	settingStore.session_before_next_long_break);

const paused = computed(() => settingStore.paused);
const pauseIcon = computed(() => paused.value? mdiPlay:mdiPause);
const pauseText = computed(() => paused.value? 'resume':'pause');

const switches = computed(() => [
	{
		label: 'fullscreen',
		model: fullscreen
	},
	{
		label: 'start on boot',
		model: start_on_boot
	}

]
);

const sliders = computed(() => [
	{
		name: 'session length',
		model: session_as_sec,
		min: 60,
		step: 60,
		max: 60 * 59,
		label_value: sec_to_minutes_only(session_as_sec.value)
	},
	{
		name: 'short break length',
		model: short_break_as_sec,
		min: 10,
		step: 10,
		max: 60 * 2,
		label_value: sec_to_minutes(short_break_as_sec.value)
	},
	{
		name: 'long break length',
		model: long_break_as_sec,
		min: 60,
		step: 15,
		max: 60 * 10,
		label_value: sec_to_minutes(long_break_as_sec.value)
	},
	{
		name: 'sessions before long break',
		model: number_session_before_break,
		min: 2,
		step: 1,
		max: 10,
		label_value: number_session_before_break.value
	}
]);

const start_on_boot = computed({
	get(): boolean {
		return settingStore.autostart;
	},
	set(b: boolean) {
		settingStore.set_autostart(b);
	}
});

const fullscreen = computed({
	get(): boolean {
		return settingStore.fullscreen;
	},
	set(b: boolean) {
		settingStore.set_fullscreen(b);
	}
});

const session_as_sec = computed({
	get(): number {
		return settingStore.session_as_sec;
	},
	set(b: number) {
		settingStore.set_session_as_sec(b);
	}
});

const short_break_as_sec = computed({
	get(): number {
		return settingStore.short_break_as_sec;
	},
	set(b: number) {
		settingStore.set_short_break_as_sec(b);
	}
});

const long_break_as_sec = computed({
	get(): number {
		return settingStore.long_break_as_sec;
	},
	set(b: number) {
		settingStore.set_long_break_as_sec(b);
	}
});

const number_session_before_break = computed({
	get(): number {
		return settingStore.number_session_before_break;
	},
	set(b: number) {
		settingStore.set_number_session_before_break(b);
	}
});

const reset_settings = async (): Promise<void> => {
	clearInterval(saveTimeout.value);
	await invoke(InvokeMessage.ResetSettings);
};

const toggle_pause = async (): Promise<void> => {
	await invoke(InvokeMessage.TogglePause);
};

const send_settings = async (message: InvokeMessage, value: number | boolean): Promise<void> => {
	saveTimeout.value = window.setTimeout(async () => {
		try {
			await invoke(message, { value });
		} catch (e) {
			snackError({ message: 'Unable to save settings' });
		}
	}, 250);
};

const saveTimeout = ref(0);

watch(fullscreen, async (value) => {
	await send_settings(InvokeMessage.SetSettingFullscreen, value);
});

watch(long_break_as_sec, async (value) => {
	await send_settings(InvokeMessage.SetSettingLongBreak, value);
});

watch(session_as_sec, async (value) => {
	await send_settings(InvokeMessage.SetSettingSession, value);
});

watch(short_break_as_sec, async (value) => {
	await send_settings(InvokeMessage.SetSettingShortBreak, value);
});

watch(number_session_before_break, async (value) => {
	await send_settings(InvokeMessage.SetSettingNumberSession, value);
});

watch(start_on_boot, async (value) => {
	await send_settings(InvokeMessage.SetAutoStart, value);
});

</script>

<style>

.v-label {
	opacity: 1 !important;
}

.hr {
	border: 1px solid rgba(var(--v-theme-primary), .9);
}

.v-switch__track {
	background-color: rgb(var(--v-theme-offwhite))!important;
}
</style>