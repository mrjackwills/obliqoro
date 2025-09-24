<template>
	<v-row class='ma-0 pa-0' justify='center' align='center'>
		<v-chip color='primary' variant='flat' size='x-large'>
			<v-col cols='auto' class='ma-0 pa-0 mr-1'>
				<v-icon :icon='mdiStarShooting' color='black' />
			</v-col>
			<v-col cols='auto' class=' text-black ma-0 pa-0 mr-1'>
				A new version is available
			</v-col>
			<v-col cols='auto' class=' text-black ma-0 pa-0 mr-1'>
				-
			</v-col>
			<v-col cols='auto' class=' text-black ma-0 pa-0 cl'>
				<span @click='openHref'>visit download page</span>
				<v-icon :icon='mdiLinkVariant ' color='black' size='x-small' />
			</v-col>
		</v-chip>
	</v-row>
</template>


<script setup lang="ts">
import { InvokeMessage } from '@/types';
import { mdiLinkVariant, mdiStarShooting } from '@mdi/js';
import { invoke } from '@tauri-apps/api/core';

const openHref = async (): Promise<void> => {
	await invoke(InvokeMessage.OpenLocation, { location: href.value });
};

const href = computed(() => `${packageinfoModule().homepage}obliqoro/releases/latest`);
</script>
