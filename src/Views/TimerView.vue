<template>
	<v-container fluid class='ma-0 pa-0 no-gutters fill-height'>
		<v-row align='center' justify='center' class='fill-height ma-0 pa-0'>
			<v-container fluid class='ma-0 pa-0'>
				<v-row align='center' justify='center' class='ma-0 pa-0'>
					<v-col cols='auto' class='ma-0 pa-0'>
						<v-progress-circular :model-value='circ_value' color='primary' :size='circle_size' width='30'>
							<v-row align='center' justify='center' class='ma-0 pa-0'>
								<v-col cols='11' class='text-primary text-center ma-2' :class='text_size'>
									{{ strategy }}
								</v-col>
								<v-col cols='auto' class='text-subtitle1 monospace-text text-primary'>
									{{ sec_to_minutes(timeout) }}
								</v-col>
							</v-row>
						</v-progress-circular>
					</v-col>
				</v-row>
			</v-container>
		</v-row>
	</v-container>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { FrontEndRoutes } from '../types';
import { sec_to_minutes } from '../vanillaTS/second';

const store = intervalModule();
const timeout = computed(() => store.interval);
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

watch(timeout, (i) => {
	if (i <= 0) {
		// set a 200ms timeout?
		router.push(FrontEndRoutes.Settings);
	}

});

</script>