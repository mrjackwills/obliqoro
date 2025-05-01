<template>
	<v-row class='ma-0 pa-0 mt-12' justify='center'>
		<v-col cols='auto' class='ma-0 pa-0 mt-12'>
			<!-- :disabled='paused' -->
			<v-btn @click='reset_settings'  variant='outlined' color='red' block rounded='sm'>
				<v-icon :icon='mdiCogRefresh' class='mr-1' />
				reset settings
			</v-btn>
		</v-col>
	</v-row>
</template>
<script setup lang="ts">
import { mdiCogRefresh } from '@mdi/js';
import { InvokeMessage } from '../../types';
import { invoke } from '@tauri-apps/api';

const reset_settings = async (): Promise<void> => {
	clearInterval(props.saveTimeout);
	await invoke(InvokeMessage.ResetSettings);
	if (settingModule().paused) await invoke(InvokeMessage.TogglePause);
};

const props = defineProps<{ saveTimeout: number }>();

</script>