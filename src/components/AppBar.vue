<template>
	<v-app-bar app color='transparent' class='elevation-0'>
		<v-row align='center' justify='space-between'>

			<v-col cols='auto' class='ml-3'>
				<v-row align='center' justify='start'>
					<v-col cols='auto' class=''>
						<v-img src='@/assets/logo_transparent.svg' :eager='true' contain width='3rem' class='' />
					</v-col>
					<v-col cols='auto' class='ma-0 pa-0'>
						<span class='text-h4 text-primary'>Obliqoro</span>

					</v-col>
				</v-row>
			</v-col>
			<v-col cols='auto' class='mr-3'>
				<v-icon :icon='mdiMinusThick' size='x-large' color='primary' @click='minimize' />
				<v-tooltip activator='parent' v-if='show_tooltip' location='left center' content-class='tooltip'>
					close to system tray
				</v-tooltip>

			</v-col>
		</v-row>
	</v-app-bar>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { InvokeMessage } from '@/types';
import { mdiMinusThick } from '@mdi/js';

const show_tooltip = ref(true);

const minimize = async (): Promise<void> => {
	show_tooltip.value = false;
	await invoke(InvokeMessage.Minimize);
	setTimeout(() => {
		show_tooltip.value = true;
	}, 100);
};

</script>

<style>
#obliqoro {
	overflow: hidden !important;
}
</style>
