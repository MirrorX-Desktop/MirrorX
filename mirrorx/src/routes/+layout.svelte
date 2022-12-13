<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { loadAllLocalesAsync } from '$lib/i18n/i18n-util.async';
	import { loadAllLocales } from '$lib/i18n/i18n-util.sync';

	loadAllLocales();

	onMount(async () => {
		if (import.meta.env.PROD) {
			document.addEventListener('contextmenu', (event) => event.preventDefault());
		}
	});

	navigator.platform.toLowerCase().includes('mac');
</script>

<slot />
