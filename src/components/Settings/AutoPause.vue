<template>
	<v-row class='text-primary ma-0 pa-0' justify='space-between'>

		<v-col cols='auto' class='ma-0 pa-0'>
			<v-row class='text-primary ma-0 pa-0' justify='start'>

				<v-col cols='auto' class='text-center ma-0 pa-0 mr-3' >
					<v-switch v-model='auto_pause' base-color='offwhite' color='primary' density='compact' flat/>
				</v-col>
				
				<v-col cols='auto' class='ma-0 pa-0 text-left text-body-2 mt-2'
					:class='auto_pause ? "text-primary" : "text-offwhite"'>
					auto-pause
				</v-col>
			
			</v-row>
			<v-tooltip activator='parent' v-if='!auto_pause' content-class='tooltip' text='Automatically pause session if average CPU usage drops below threshold' />
			
			
		</v-col>

		<v-spacer />

		<v-col cols='5' class='ma-0 pa-0 text-left mt-2' v-if='auto_pause'
			:class='[!auto_pause ? "text-offwhite diss" : auto_pause ? "text-primary" : "text-offwhite"]'>
			<v-row class='ma-0 pa-0 text-caption' justify='space-between'>
				<v-col cols='auto' class='ma-0 pa-0'>
					<span class='text-offwhite '>average from previous
						<span class='mono-num'>{{ secondsToText(auto_pause_timespan_sec, true) }}</span>
					</span>
				</v-col>
				<v-col cols='auto' class='ma-0 pa-0'>
					<span v-if='average_pause === 0'>
						<v-progress-circular indeterminate class='ml-2 mt-n1' color='primary' size='10' width='1' />
					</span>
					<span v-else class='mono-num'>
						{{ formatPercentage(average_pause) }}%
					</span>
				</v-col>
			</v-row>
		</v-col>

	</v-row>

	<v-row class='text-primary ma-0 pa-0 mt-n6' :class='{ "disabled-opacity": !auto_pause }' justify='space-between'>

		<v-col cols='5' v-for='(item, index) in sliders_auto_pause' :key='index' class='ma-0 pa-0'>

			<v-row class='text-offwhite ma-0 pa-0'>
				<v-col cols='auto' class='ma-0 pa-0'>
					{{ item.name }}
				</v-col>
				<v-spacer />
				<v-col cols='auto' class='ma-0 pa-0' :class='{ "text-primary": auto_pause }'>
					{{ item.label_value }}
				</v-col>
			</v-row>

			<v-row class='ma-0 pa-0 mt-n2'>
				<v-col cols='12' class='ma-0 pa-0'>
					<v-slider v-model='item.model.value' color='primary' :disabled='!auto_pause' :min='item.min'
						:max='item.max' :step='item.step' rounded :thumb-size='10' :track-size='2' class='ma-0 pa-0' />
				</v-col>
			</v-row>
		</v-col>
	</v-row>
</template>

<script setup lang="ts">
import { formatPercentage, secondsToText, zeroPad } from '../../vanillaTS/second';

const cpuUsageStore = cpuUsageModule();
const average_pause = computed(() => cpuUsageStore.average_pause);


const settingStore = settingModule();

const sliders_auto_pause = computed(() => [
	{
		name: 'timespan',
		model: auto_pause_timespan_sec,
		min: 60,
		step: 60,
		max: 60 * 15,
		label_value: secondsToText(auto_pause_timespan_sec.value, true)
	},
	{
		name: 'threshold',
		model: auto_pause_threshold,
		min: 1,
		step: 1,
		max: 50,
		label_value: `${zeroPad(auto_pause_threshold.value)}%`
	}
]);

const auto_pause = computed({
	get (): boolean {
		return settingStore.auto_pause;
	},
	set (b: boolean) {
		settingStore.set_auto_pause(b);
	}
});

const auto_pause_threshold = computed({
	get (): number {
		return settingStore.auto_pause_threshold;
	},
	set (b: number) {
		settingStore.set_auto_pause_threshold(b);
	}
});

const auto_pause_timespan_sec = computed({
	get (): number {
		return settingStore.auto_pause_timespan_sec;
	},
	set (b: number) {
		settingStore.set_auto_pause_timespan_sec(b);
	}
});

</script>
