<template>

	<v-row class='text-primary ma-0 pa-0 mt-n6' justify='space-between'>
		
		<v-col cols='3' class='text-center ma-0 pa-0 text-caption'>
	
			<v-row class='ma-0 pa-0' justify='space-between'>
				<v-col cols='auto' class='ma-0 pa-0'>
					<span class='text-offwhite'> current CPU usage: </span>
				</v-col>
				<v-col cols='auto' class='ma-0 pa-0 mono-num'>
					{{ formatPercentage(current) }}%
				</v-col>
			</v-row>
		</v-col>
		
		<v-col cols='3' class='text-center ma-0 pa-0 text-caption'>
			
			<v-row class='ma-0 pa-0' justify='space-between'>
				<v-col cols='auto' class='ma-0 pa-0'>
					<span class='text-offwhite'><span class='mono-num'>{{ secondsToText(auto_pause_timespan_sec, true) }}</span> pause average CPU usage: </span>
				</v-col>
				<v-col cols='auto' class='ma-0 pa-0'>
					<span v-if='average_pause === 0'>
						<v-progress-circular
							indeterminate
							class='ml-2 mt-n1'
							color='primary'
							size='10'
							width='1'
						/> 
					</span>
					<span v-else class='mono-num'>
						{{ formatPercentage(average_pause) }}%
					</span>
				</v-col>
			</v-row>
		</v-col>

		<v-col cols='3' class='text-center ma-0 pa-0 text-caption'>
			
			<v-row class='ma-0 pa-0' justify='space-between'>
				<v-col cols='auto' class='ma-0 pa-0'>
					<span class='text-offwhite'><span class='mono-num'>{{ secondsToText(auto_pause_timespan_sec, true) }}</span> resume average CPU usage: </span>
				</v-col>
				<v-col cols='auto' class='ma-0 pa-0'>
					<span v-if='average_resume === 0'>
						<v-progress-circular
							indeterminate
							class='ml-2 mt-n1'
							color='primary'
							size='10'
							width='1'
						/> 
					</span>
					<span v-else class='mono-num'>
						{{ formatPercentage(average_resume) }}%
					</span>
				</v-col>
			</v-row>
		</v-col>
	</v-row>
</template>


<script setup lang="ts">
import { formatPercentage, secondsToText } from '../../vanillaTS/second';

const cpuUsageStore = cpuUsageModule();
const current = computed(() => cpuUsageStore.current);
const average_pause = computed(() => cpuUsageStore.average_pause);
const average_resume = computed(() => cpuUsageStore.average_resume);

const auto_pause_timespan_sec = computed(() => settingModule().auto_pause_timespan_sec);

</script>

