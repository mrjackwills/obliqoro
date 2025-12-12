<template>
	<v-tooltip v-if='paused' activator='parent' content-class='tooltip' location='top center'>
		resume to alter
	</v-tooltip>
</template>

<script setup lang="ts">
defineProps<{ paused: boolean }>()

</script>
