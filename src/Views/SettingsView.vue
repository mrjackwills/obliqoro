<template>
	<v-row class='ma-0 pa-0 no-gutters fill-height' align='center'>

		<v-row align='center' justify='center' class='ma-0 pa-0'>
			<v-col cols='9' class='ma-0 pa-0'>

				<v-row align='center' justify='center' class='ma-0 pa-0'>
					<v-col cols='auto' class='text-h4 ma-0 pa-0 text-primary'>
						Settings
					</v-col>
				</v-row>

				<HR />

				<PauseRow />

				<HR />

				<!-- SWITCHES -->
				<v-row align='center' class='ma-0 pa-0' justify='space-between'>

					<v-col v-for='(item, index) in switches' :key='index' cols='auto' class='ma-0 pa-0'>

						<v-row class='text-primary ma-0 pa-0' justify='space-between'>

							<v-col cols='auto' class='text-center ma-0 pa-0 mr-3'>
								<v-switch v-model='item.model.value' base-color='offwhite' color='primary'
									density='compact' flat />
							</v-col>

							<v-col cols='auto' class='ma-0 pa-0 text-left text-body-2 mt-2'
								:class='item.model.value ? "text-primary" : "text-offwhite"'>
								{{ item.label }}
							</v-col>

						</v-row>

					</v-col>

				</v-row>

				<SessionBreakSliders />
				<AutoPause :rotation />
				<AutoResume :rotation />
				<v-expand-transition>
					<VersionAlert v-if='show_update' />
				</v-expand-transition>
				<ResetButton :saveTimeout='saveTimeout' />
			</v-col>
		</v-row>

	</v-row>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { InvokeMessage } from '@/types';
import { snackError } from '@/services/snack';
import SessionBreakSliders from '@/components/Settings/SessionBreakSliders.vue';
const settingStore = settingModule();

const show_update = computed(() => packageinfoModule().github_version.length > 1 && packageinfoModule().version !== packageinfoModule().github_version);

// / Pass is rotation as a prop, so that both spinners have the same animation
const rotation = ref(0);
const rotation_interval = ref(0);

// TODO this can pause if the timer view appears before it's calculated
const start_rotation_interval = (): void => {
	rotation_interval.value = window.setInterval(() => {
		rotation.value += 20;
		if (rotation.value >= 360) rotation.value = 0;
	}, 30);
};

const stop_rotation_interval = (): void => {
	clearInterval(rotation_interval.value);
};

const run_rotation = computed(() => settingStore.auto_pause && cpuUsageModule().average_pause === 0 || settingStore.auto_resume && cpuUsageModule().average_resume === 0);

onMounted(() => {
	start_rotation_interval();
});

onUnmounted(() => {
	stop_rotation_interval();
});

// / Only run rotation interval if needed
watch(run_rotation, (i) => {
	if (i) {
		start_rotation_interval();
	} else {
		stop_rotation_interval();
	}
});

const switches = computed(() => [
	{
		label: 'fullscreen',
		model: fullscreen
	},

	{
		label: 'start on boot',
		model: start_on_boot
	}
]);

const start_on_boot = computed({
	get (): boolean {
		return settingStore.start_on_boot;
	},
	set (b: boolean) {
		settingStore.set_start_on_boot(b);
	}
});

const fullscreen = computed({
	get (): boolean {
		return settingStore.fullscreen;
	},
	set (b: boolean) {
		settingStore.set_fullscreen(b);
	}
});

const saveTimeout = ref(0);

const current_state = computed(() => settingStore.get_current_state);

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


watch(current_state, async () => {
	await send_state();
});

</script>
