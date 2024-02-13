<template>
	<v-row align='center' justify='center' class='ma-0 pa-0 fill-height'>
		<v-col cols='auto' class='ma-0 pa-0' v-if='interval > 0'>
			<v-progress-circular :model-value='circ_value' color='primary' :size='circle_size' width='30' class=''>

				<v-container fluid class='pa-0'>
								
					<v-row align='center' justify='center' class='mx-1 mb-12 switch_margin'>
						<v-col cols='11' class='text-primary text-center ma-0 pa-0 px-2 mb-6' :class='text_size'>
							{{ strategy }}
						</v-col>
								
						<v-col cols='auto' class='text-subtitle-1 monospace-text text-primary ma-0 pa-0'>
							{{ sec_to_minutes(interval) }}
						</v-col>

					</v-row>
					<v-row align='center' justify='center' class='ma-0 pa-0'>
						<v-col cols='auto' class='ma-0 pa-0'>
							<v-switch
								v-model='pauseAfterBreak'
								:class='pauseAfterBreak ? "text-primary" : "text-offwhite"'
								class='ma-0 pa-0'
								color='primary'
								density='compact'
								hide-details
								flat
							/>
						</v-col>
						<v-col cols='auto' class='mb-1 ml-4 text-subtitle-2 text-primary ma-0 pa-0' :class='pauseAfterBreak ? "text-primary" : "text-offwhite"'>
							pause after break
						</v-col>
					</v-row>
				</v-container>

			</v-progress-circular>
				
		</v-col>
	
	</v-row>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { FrontEndRoutes, InvokeMessage } from '../types';
import { sec_to_minutes } from '../vanillaTS/second';
import { invoke } from '@tauri-apps/api/tauri';

const store = intervalModule();
const interval = computed(() => store.interval);

const circ_value = computed(() => {
	return store.interval * (100 / store.original_interval);
});

const strategy = computed((): string => {
	return store.strategy;
});

const circle_size = computed((): string => {
	return settingModule().fullscreen ? '1000' : '800';
});

const text_size = computed((): string => {
	return settingModule().fullscreen ? 'text-h3' : 'text-h4';
});

const router = useRouter();

const pauseAfterBreak = ref(false);

watch(pauseAfterBreak, async (pause) => {
	await invoke(InvokeMessage.PauseAfterBreak, { pause });
});

watch(interval, async (i) => {
	if (i <= 0) {
		// set a 200ms timeout?
		router.push(FrontEndRoutes.Settings);
	}
});

</script>

<style>

/* This is to account for the the height of the switch row - it might not be perfect */
.switch_margin {
	margin-top: 18%;
}

</style>
