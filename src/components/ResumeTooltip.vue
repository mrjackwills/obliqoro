<template>

	<v-tooltip activator='parent' v-if='paused' location='top center' content-class='tooltip'>
		resume to alter
	</v-tooltip>
</template>

<script setup lang="ts">
defineProps<{paused: boolean}>();

</script>