<script lang="ts">
	import { locale, setLocale } from '../i18n/i18n-svelte';
	import { baseLocale, loadedLocales, locales } from '../i18n/i18n-util';
	import { detectLocale, navigatorDetector } from 'typesafe-i18n/detectors';
	import '../app.css';
	import { loadAllLocalesAsync, loadLocaleAsync } from '../i18n/i18n-util.async';
	import { loadAllLocales } from '../i18n/i18n-util.sync';

	let detectedLocale = detectLocale(baseLocale, locales, navigatorDetector);

	loadAllLocales();
	console.log(loadedLocales);
	setLocale(detectedLocale);
</script>

<slot />
