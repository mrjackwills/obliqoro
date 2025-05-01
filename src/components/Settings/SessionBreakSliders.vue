<template>
	<v-row align='center' justify='space-between' class='ma-0 pa-0'>

		<v-col cols='5' v-for='(item, index) in sliders_settings' :key='index' class='ma-0 pa-0 my-n1' :class='{"disabled-opacity":paused}'>

			<v-row class='text-primary ma-0 pa-0'>
				<v-col cols='auto' class='text-offwhite ma-0 pa-0'>
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
						:max='item.max' :step='item.step' rounded  :thumb-size='10' :track-size='2' 
						class='ma-0 pa-0' />
					<ResumeTooltip :paused='paused' />
				</v-col>
			</v-row>
		</v-col>

	</v-row>
</template>

<script setup lang="ts">

import { secondsToText } from '../../vanillaTS/helpers';

const settingStore = settingModule();

const sliders_settings = computed(() => [
	{
		name: 'session length',
		model: session_as_sec,
		min: 60,
		step: 60,
		max: 60 * 120,
		label_value: secondsToText(session_as_sec.value, true)
	},
	{
		name: 'sessions before long break',
		model: number_session_before_break,
		min: 2,
		step: 1,
		max: 10,
		label_value: number_session_before_break.value
	},
	{
		name: 'short break length',
		model: short_break_as_sec,
		min: 30,
		step: 30,
		max: 60 * 15,
		label_value: secondsToText(short_break_as_sec.value, false)
	},
	{
		name: 'long break length',
		model: long_break_as_sec,
		min: 60,
		step: 30,
		max: 60 * 30,
		label_value: secondsToText(long_break_as_sec.value, false)
	}
	
]);

const paused = computed(() => settingStore.paused);

const session_as_sec = computed({
	get (): number {
		return settingStore.session_as_sec;
	},
	set (b: number) {
		settingStore.set_session_as_sec(b);
	}
});

const short_break_as_sec = computed({
	get (): number {
		return settingStore.short_break_as_sec;
	},
	set (b: number) {
		settingStore.set_short_break_as_sec(b);
	}
});

const long_break_as_sec = computed({
	get (): number {
		return settingStore.long_break_as_sec;
	},
	set (b: number) {
		settingStore.set_long_break_as_sec(b);
	}
});

const number_session_before_break = computed({
	get (): number {
		return settingStore.number_session_before_break;
	},
	set (b: number) {
		settingStore.set_number_session_before_break(b);
	}
});

</script>
