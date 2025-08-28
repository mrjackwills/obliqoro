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
			<v-col cols='auto' class=' text-black ma-0 pa-0'>
				<a :href='href' target='_blank' rel='noopener noreferrer' class='text-black'>visit download page
					<v-icon :icon='mdiLinkVariant ' color='black' size='x-small' />
				</a>
			</v-col>
		</v-chip>
	</v-row>
</template>


<script setup lang="ts">
import { mdiLinkVariant, mdiStarShooting } from '@mdi/js';

const href = computed(() => `${packageinfoModule().homepage}obliqoro/releases/latest`);
</script>
