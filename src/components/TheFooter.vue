<template>
	<v-footer color='transparent' id='footer' absolute app class='ma-0 pa-0'>
		<v-row justify='center' align='center' class='no-gutters ma-0 pa-0 ma-4'>

			<v-col cols='auto' class='no-gutters unselectable ma-0 pa-0'>

				<v-chip :ripple='false' color='offwhite' text-color='black' variant='flat' outlined pill>
					<section v-if='showBuild' class='' @click='buildInfo'>
						<span>version: {{ appVersion }}</span>
						<span class='ml-3 '>built: {{ buildDate }}</span>
					</section>

					<section v-else>
						<a :href='href' target='_blank' rel='noopener noreferrer' class='text-caption'>
							<v-icon color='black' class='mr-2' href='' :icon='mdiGithub' />
						</a>
						<span @click='buildInfo' class=''>
							mrjackwills 2022 -
						</span>
					</section>

				</v-chip>
			</v-col>
		</v-row>

	</v-footer>
</template>

<script setup lang='ts'>

import { mdiGithub } from '@mdi/js';

const buildTimeout = ref(0);
const showBuild = ref(false);

const appVersion = computed((): string => {
	return packageinfoModule().version;
});

const buildDate = computed((): string => {
	return new Date(Number(packageinfoModule().build_date) * 1000).toISOString();
});

const href = computed((): string => {
	return packageinfoModule().homepage;
});

onUnmounted(() => {
	clearTimeout(buildTimeout.value);
});

const buildInfo = (): void => {
	showBuild.value = !showBuild.value;
	clearTimeout(buildTimeout.value);
	if (showBuild.value) {
		buildTimeout.value = window.setTimeout(() => {
			showBuild.value = false;
		}, 10000);
	}
};

</script>

<style scoped>
.lowercase-button {
	text-transform: lowercase;
}

a {
	color: #000000 !important;
	text-decoration: none;
}
</style>