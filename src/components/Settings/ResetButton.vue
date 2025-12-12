<template>
	<v-row class='ma-0 pa-0 mt-12' justify='center'>
		<v-col class='ma-0 pa-0 mt-12' cols='auto'>
			<v-btn
				block
				color='red'
				rounded='sm'
				variant='outlined'
				@click='reset_settings'
			>
				<v-icon class='mr-1' :icon='mdiCogRefresh' />
				reset settings
			</v-btn>
		</v-col>
	</v-row>
</template>
<script setup lang="ts">
import { mdiCogRefresh } from '@mdi/js'
import { invoke } from '@tauri-apps/api/core'
import { InvokeMessage } from '@/types'

async function reset_settings (): Promise<void> {
	clearInterval(props.saveTimeout)
	await invoke(InvokeMessage.ResetSettings)
	if (settingModule().paused) await invoke(InvokeMessage.TogglePause)
}

const props = defineProps<{ saveTimeout: number }>()

</script>
