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

			</v-col>
		</v-row>
	</v-app-bar>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { InvokeMessage } from '../types';
import { mdiMinusThick } from '@mdi/js';

const minimize = async (): Promise<void> => {
	await invoke(InvokeMessage.Minimize);
};

</script>

<style>
#obliqoro {
	overflow: hidden !important;
}
</style>
