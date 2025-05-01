<template>
	<v-snackbar v-model='visible' timeout='-1' class='mb-12' color='red' location='bottom'>
		<v-row justify='center' align='center' no-gutters class='ma-0 pa-0'>
			<v-col v-if='icon && !loading' cols='auto' class=''>
				<v-icon color='black' :icon='icon' />
			</v-col>
			<v-col v-if='loading' cols='auto' class=''>
				<v-progress-circular :size='18' :width='3' color='black' indeterminate />
			</v-col>
			<v-col cols='auto' class='mx-3 ma-0 pa-0 text-black'>
				{{ message }}
			</v-col>
		</v-row>
	</v-snackbar>
</template>

<script setup lang="ts">
const snackStore = snackModule();
const loading = computed({
	get (): boolean {
		return snackStore.loading;
	},
	set (b: boolean) {
		snackStore.set_loading(b);
	}
});

const message = computed({
	get (): string {
		return snackStore.message;
	},
	set (b: string) {
		snackStore.set_message(b);
	}
});

const snackTimeout = computed({
	get (): number {
		return snackStore.timeout;
	},
	set (n: number) {
		snackStore.set_timeout(n);
	}
});

const visible = computed({
	get (): boolean {
		return snackStore.visible;
	},
	set (b: boolean) {
		snackStore.set_visible(b);
	}
});

const timeout = ref(0);

const icon = computed(() => snackStore.icon);

const closeSnackbar = (): void => {
	visible.value = false;
	snackStore.$reset();
	clearTimeout(timeout.value);
	timeout.value = 0;
};

watch(visible, (i) => {
	if (i && snackTimeout) {
		timeout.value = window.setTimeout(() => {
			closeSnackbar();
		}, snackTimeout.value);
	}
});
</script>
