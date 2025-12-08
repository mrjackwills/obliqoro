<template>
	<v-row align='center' class='ma-0 pa-0' justify='center'>
		<v-chip color='primary' size='x-large' variant='flat'>
			<v-col class='ma-0 pa-0 mr-1' cols='auto'>
				<v-icon color='black' :icon='mdiStarShooting' />
			</v-col>
			<v-col class=' text-black ma-0 pa-0 mr-1' cols='auto'>
				A new version is available
			</v-col>
			<v-col class=' text-black ma-0 pa-0 mr-1' cols='auto'>
				-
			</v-col>
			<v-col class=' text-black ma-0 pa-0 cl' cols='auto'>
				<span @click='openHref'>visit download page</span>
				<v-icon color='black' :icon='mdiLinkVariant ' size='x-small' />
			</v-col>
		</v-chip>
	</v-row>
</template>

<script setup lang="ts">
import { mdiLinkVariant, mdiStarShooting } from '@mdi/js'
import { invoke } from '@tauri-apps/api/core'
import { InvokeMessage } from '@/types'

async function openHref (): Promise<void> {
	await invoke(InvokeMessage.OpenLocation, { location: href.value })
}

const href = computed(() => `${packageinfoModule().homepage}obliqoro/releases/latest`)
</script>
