<template>

	<v-tooltip activator='parent' v-if='paused' location='top center' class='tooltip-z'>
		resume to alter
	</v-tooltip>
</template>

<script setup lang="ts">
defineProps({
	paused: {
		type: Boolean,
		required: true
	}
});

</script>