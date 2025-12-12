<template>
	<v-footer
		id='footer'
		absolute
		app
		class='ma-0 pa-0'
		color='transparent'
	>
		<v-row align='center' class='no-gutters ma-0 pa-0 ma-4' justify='space-between'>
			<v-col class='ma-0 pa-0' cols='auto'>
				<v-row v-if='showBuild && GitHubVersion' align='center' class='ma-0 pa-0' justify='start'>
					<v-col class='ma-0 pa-0 text-primary text-caption' cols='auto'>
						latest GitHub version: {{ GitHubVersion }}
					</v-col>
				</v-row>
			</v-col>

			<v-col class='no-gutters unselectable ma-0 pa-0 cl' cols='auto'>

				<v-chip
					color='offwhite'
					outlined
					pill
					:ripple='false'
					text-color='black'
					variant='flat'
				>
					<section v-if='showBuild' class='' @click='buildInfo'>
						<span>version: {{ appVersion }}</span>
						<span class='ml-3 '>built: {{ buildDate }}</span>
					</section>

					<section v-else>
						<v-icon class='mr-2 mt-n1' color='black' :icon='mdiGithub' @click='openHref' />
						<span class='' @click='buildInfo'>
							mrjackwills 2022 -
						</span>
					</section>

				</v-chip>
			</v-col>
			<v-col class='ma-0 pa-0 text-caption text-primary' cols='auto'>
				<v-row
					v-if='showBuild'
					align='center'
					class='ma-0 pa-0 cl'
					justify='end'
					@click='opendb'
				>
					<v-col class='ma-0 pa-0' cols='auto'>
						database location
					</v-col>
					<v-col class='ma-0 pa-0 ml-1' cols='auto'>
						<v-icon color='primary' :icon='mdiOpenInNew' size='x-small' />
					</v-col>
				</v-row>
				<v-tooltip activator='parent' content-class='tooltip' location='top center'>
					open database location in file explorer
				</v-tooltip>
			</v-col>
		</v-row>

	</v-footer>
</template>

<script setup lang='ts'>

import { mdiGithub, mdiOpenInNew } from '@mdi/js'
import { invoke } from '@tauri-apps/api/core'
import { InvokeMessage } from '@/types'

const buildTimeout = ref(0)
const showBuild = ref(false)

const appVersion = computed(() => packageinfoModule().version)
const GitHubVersion = computed(() => packageinfoModule().github_version)

const buildDate = computed(() => new Date(Number(packageinfoModule().build_date) * 1000).toISOString())

async function opendb (): Promise<void> {
	await invoke(InvokeMessage.OpenLocation)
}

async function openHref (): Promise<void> {
	await invoke(InvokeMessage.OpenLocation, { location: href.value })
}

const href = computed(() => packageinfoModule().homepage)

onUnmounted(() => {
	clearTimeout(buildTimeout.value)
})

function buildInfo (): void {
	showBuild.value = !showBuild.value
	clearTimeout(buildTimeout.value)
	if (showBuild.value) {
		buildTimeout.value = window.setTimeout(() => {
			showBuild.value = false
		}, 10_000)
	}
}

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
