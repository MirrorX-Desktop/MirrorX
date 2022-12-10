<script lang="ts">
	import '../app.css';
	import { detectLocale, isLocale } from '../i18n/i18n-util';
	import { navigatorDetector } from 'typesafe-i18n/detectors';
	import { onMount } from 'svelte';
	import { loadAllLocalesAsync } from '$lib/i18n/i18n-util.async';
	import { setLocale } from '$lib/i18n/i18n-svelte';
	import { invoke_init, invoke_get_language, invoke_set_language } from '$lib/components/command';
	import type { Locales } from '$lib/i18n/i18n-types';

	onMount(async () => {
		if (import.meta.env.PROD) {
			document.addEventListener('contextmenu', (event) => event.preventDefault());
		}

		await loadAllLocalesAsync();
		await invoke_config_init();
		invoke_init;

		setLocale(lang as Locales);
	});
</script>

<slot />
