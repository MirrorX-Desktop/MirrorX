<script lang="ts">
	import '../app.css';
	import { detectLocale, isLocale } from '../i18n/i18n-util';
	import { navigatorDetector } from 'typesafe-i18n/detectors';
	import { onMount } from 'svelte';
	import { loadAllLocalesAsync } from '$lib/i18n/i18n-util.async';
	import { setLocale } from '$lib/i18n/i18n-svelte';
	import type { Locales } from '$lib/i18n/i18n-types';

	onMount(async () => {
		if (import.meta.env.PROD) {
			document.addEventListener('contextmenu', (event) => event.preventDefault());
		}

		await loadAllLocalesAsync();
	});
</script>

<slot />
