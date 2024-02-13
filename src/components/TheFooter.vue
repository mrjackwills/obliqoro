<template>
	<v-footer color='transparent' id='footer' absolute app class='ma-0 pa-0'>
		<v-row justify='space-between' align='center' class='no-gutters ma-0 pa-0 ma-4'>
			<v-col cols='2' class='ma-0 pa-0'>

			</v-col>

			<v-col cols='auto' class='no-gutters unselectable ma-0 pa-0'>

				<v-chip :ripple='false' color='offwhite' text-color='black' variant='flat' outlined pill>
					<section v-if='showBuild' class='' @click='buildInfo'>
						<span>version: {{ appVersion }}</span>
						<span class='ml-3 '>built: {{ buildDate }}</span>
					</section>

					<section v-else>
						<a :href='href' target='_blank' rel='noopener noreferrer' class='text-caption'>
							<v-icon color='black' class='mr-2' href='' :icon='mdiGithub' />
						</a>
						<span @click='buildInfo' class=''>
							mrjackwills 2022 -
						</span>
					</section>

				</v-chip>
			</v-col>
			<v-col cols='2' class='ma-0 pa-0 text-caption text-primary'>
				<section v-if='showBuild'  class='cl' @click='opendb'>
					<v-row class='ma-0 pa-0' justify='end' align='center'>
						<v-col cols='auto' class='ma-0 pa-0'>
							database location
						</v-col>
						<v-col cols='auto' class='ma-0 pa-0 ml-1'>
							<v-icon :icon='mdiOpenInNew' color='primary' size='x-small' />
						</v-col>
					</v-row>
					
				</section>
			</v-col>
		</v-row>

	</v-footer>
</template>

<script setup lang='ts'>

import { mdiGithub, mdiOpenInNew } from '@mdi/js';
import { invoke } from '@tauri-apps/api';
import { InvokeMessage } from '../types';

const buildTimeout = ref(0);
const showBuild = ref(false);

const appVersion = computed(() => packageinfoModule().version);

const buildDate = computed(() => new Date(Number(packageinfoModule().build_date) * 1000).toISOString());

const opendb = async (): Promise<void> => {
	await invoke(InvokeMessage.OpenDatabaseLocation);
};

const href = computed(() => packageinfoModule().homepage);

onUnmounted(() => {
	clearTimeout(buildTimeout.value);
});

const buildInfo = (): void => {
	showBuild.value = !showBuild.value;
	clearTimeout(buildTimeout.value);
	if (showBuild.value) {
		buildTimeout.value = window.setTimeout(() => {
			showBuild.value = false;
		}, 10000);
	}
};

</script>

<style scoped>
.lowercase-button {
	text-transform: lowercase;
}

a {
	color: #000000 !important;
	text-decoration: none;
}
</style>