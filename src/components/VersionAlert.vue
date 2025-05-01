<template>
	<v-row class='ma-0 pa-0' justify='center' align='center'>
		<v-col cols='auto' class='ma-0 pa-0'>
			<v-icon :icon='mdiStarShooting' color='primary' />
		</v-col>
		<v-col cols='auto' class=' text-primary ma-0 pa-0'>
			A new version is available
		</v-col>
		<v-col cols='auto' class=' text-primary ma-0 pa-0 mx-1'>
			- 
		</v-col>
		<v-col cols='auto' class=' text-primary ma-0 pa-0'>
			<a :href='href' target='_blank' rel='noopener noreferrer' class='text-primary'>visit download page
				<v-icon :icon='mdiOpenInNew' color='primary' size='x-small' />
			</a>
		</v-col>
	</v-row>
</template>


<script setup lang="ts">
import { mdiOpenInNew, mdiStarShooting } from '@mdi/js';

const href = computed(() => `${packageinfoModule().homepage}obliqoro/releases/latest`);
</script>