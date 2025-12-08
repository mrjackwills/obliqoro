<template>
	<v-row align='center' class='ma-0 pa-0' justify='space-between'>

		<v-col class='ma-0 pa-0 text-primary text-left' cols='5'>
			<v-row align='center' class='ma-0 pa-0' justify='start'>
				<template v-if='!paused'>
					<v-col class='ma-0 pa-0 mr-2' cols='auto'>
						<v-icon class='' :icon='mdiCoffeeOutline' />
					</v-col>
					<v-col class='ma-0 pa-0' cols='auto'>
						{{ next_in }}
					</v-col>
				</template>
			</v-row>
		</v-col>

		<v-col class='ma-0 pa-0' cols='2'>
			<v-btn
				block
				class='ma-0 pa-0'
				color='primary'
				rounded='sm'
				@click='toggle_pause'
			>
				<v-row align='center' class='ma-0 pa-0' justify='start'>
					<v-col class='ma-0 pa-0 mr-1' cols='auto'>
						<v-icon class='' :icon='pauseIcon' />
					</v-col>
					<v-col class='ma-0 pa-0' cols='auto'>
						{{ pauseText }}
					</v-col>
				</v-row>
			</v-btn>
		</v-col>

		<v-col class='ma-0 pa-0 text-primary' cols='5'>
			<v-row v-if='!paused' align='center' class='ma-0 pa-0' justify='end'>
				<v-col class='ma-0 pa-0' cols='auto'>
					{{ sessions_before_long }}
				</v-col>
				<v-col class='ma-0 pa-0 ml-2' cols='auto'>
					<v-icon class='' :icon='mdiWeatherNight' />
				</v-col>
			</v-row>

		</v-col>
	</v-row>
</template>

<script setup lang="ts">
import { mdiCoffeeOutline, mdiPause, mdiPlay, mdiWeatherNight } from '@mdi/js'
import { invoke } from '@tauri-apps/api/core'
import { snackError } from '@/services/snack'
import { InvokeMessage } from '@/types'

const settingStore = settingModule()

const next_in = computed(() => nextbreakModule().nextbreak)

const sessions_before_long = computed(() => settingStore.session_before_next_long_break)

const paused = computed({
	get (): boolean {
		return settingStore.paused
	},
	set (b: boolean) {
		settingStore.set_paused(b)
	},
})

async function toggle_pause (): Promise<void> {
	paused.value = !paused.value
	try {
		await invoke(InvokeMessage.TogglePause)
	} catch (error) {
		snackError({ message: `Unable to pause: ${error}` })
	}
}

const pauseIcon = computed(() => paused.value ? mdiPlay : mdiPause)
const pauseText = computed(() => paused.value ? 'resume' : 'pause')
</script>
