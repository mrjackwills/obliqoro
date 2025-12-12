<template>
	<v-app-bar app class='elevation-0' color='transparent'>
		<v-row align='center' justify='space-between'>

			<v-col class='ml-3' cols='auto'>
				<v-row align='center' justify='start'>
					<v-col class='' cols='auto'>
						<v-img
							class=''
							contain
							:eager='true'
							src='@/assets/logo_transparent.svg'
							width='3rem'
						/>
					</v-col>
					<v-col class='ma-0 pa-0' cols='auto'>
						<span class='text-h4 text-primary'>Obliqoro</span>

					</v-col>
				</v-row>
			</v-col>
			<v-col class='mr-3' cols='auto'>
				<v-icon color='primary' :icon='mdiMinusThick' size='x-large' @click='minimize' />
				<v-tooltip v-if='show_tooltip' activator='parent' content-class='tooltip' location='left center'>
					close to system tray
				</v-tooltip>

			</v-col>
		</v-row>
	</v-app-bar>
</template>

<script setup lang="ts">
import { mdiMinusThick } from '@mdi/js'
import { invoke } from '@tauri-apps/api/core'
import { InvokeMessage } from '@/types'

const show_tooltip = ref(true)

async function minimize (): Promise<void> {
	show_tooltip.value = false
	await invoke(InvokeMessage.Minimize)
	setTimeout(() => {
		show_tooltip.value = true
	}, 100)
}

</script>

<style>
#obliqoro {
	overflow: hidden !important;
}
</style>
