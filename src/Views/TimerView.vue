<template>
	<v-row align='center' justify='center' class='ma-0 pa-0 fill-height'>


		<v-col cols='auto' class='ma-0 pa-0' v-if='interval > 0'>
			<v-progress-circular :model-value='circ_value' color='primary' :size='circle_size' width='30' class=''>

				<v-container class='page-width pa-0 ma-0'>

					<v-row align='center' justify='center' class='mt-12'>
						<v-col cols='11' class='text-primary text-center ma-0 pa-0' :class='text_size'>
							{{ strategy }}
						</v-col>

						<v-col cols='11' class='text-center text-subtitle-1 mono-num text-primary'>
							{{ sec_to_minutes(interval) }}
						</v-col>

					</v-row>

					<v-row align='center' justify='center' class='mt-6 ma-0 pa-0'>
						<v-col cols='11' class='ma-0 pa-0'>

							<v-row align='center' justify='space-evenly' class='ma-0 pa-0'>

								<v-col cols='auto' class='text-center text-subtitle-2 text-primary ma-0 pa-0'
									:class='pauseAfterBreak ? "text-primary" : "text-offwhite"'>
									<v-switch v-model='pauseAfterBreak' base-color='offwhite' color='primary'
										density='compact' hide-details flat label='pause after break' />

								</v-col>

								<v-col cols='auto' class='text-center text-subtitle-2 text-primary ma-0 pa-0'
									:class='[{ "disabled-opacity": resume_disabled }, auto_resume ? "text-primary" : "text-offwhite"]'>
									<v-switch v-model='auto_resume' :disabled='resume_disabled' base-color='offwhite'
										color='primary' density='compact' hide-details flat label='auto resume' />

								</v-col>
							</v-row>

						</v-col>
					</v-row>

				</v-container>

			</v-progress-circular>

		</v-col>

	</v-row>
</template>

<script setup lang="ts">
import { FrontEndRoutes, InvokeMessage } from '@/types';
import { sec_to_minutes } from '@/vanillaTS/helpers';
import { invoke } from '@tauri-apps/api/core';
import { snackError } from '@/services/snack';
import { getCurrentWindow } from '@tauri-apps/api/window';

const settingStore = settingModule();

const store = intervalModule();
const interval = computed(() => store.interval);

const circ_value = computed(() => store.interval * (100 / store.original_interval));

const strategy = computed(() => store.strategy);

const circle_size = computed(() => settingModule().fullscreen ? '1000' : '800');

const text_size = computed(() => settingModule().fullscreen ? 'text-h3' : 'text-h4');

const resume_disabled = computed(() => !(settingStore.auto_pause && pauseAfterBreak.value));

const router = useRouter();
const auto_resume = computed({
	get (): boolean {
		return settingStore.auto_resume;
	},
	set (b: boolean) {
		settingStore.set_auto_resume(b);
	}
});

watch(auto_resume, async () => {
	await send_state();
});

const current_state = computed(() => settingStore.get_current_state);

// TODO: refactor this, as it's used in the SettingsView as well
const saveTimeout = ref(0);

const send_state = async (): Promise<void> => {
	if (saveTimeout.value) {
		clearTimeout(saveTimeout.value);
	}
	saveTimeout.value = window.setTimeout(async () => {
		try {
			await invoke(InvokeMessage.SetSettings, { value: current_state.value });
		} catch {
			snackError({ message: `Unable to save settings` });
		}
	}, 100);
};

const pauseAfterBreak = ref(false);

watch(pauseAfterBreak, async (pause) => {
	await invoke(InvokeMessage.PauseAfterBreak, { pause });
	settingModule().set_paused(pause);
});

watch(interval, async (i) => {
	if (i <= 0) {
		await getCurrentWindow().setFullscreen(false);
		await router.push(FrontEndRoutes.Settings);
	}
});

</script>

<style>
.page-width {
	width: 60vw;
}
</style>
