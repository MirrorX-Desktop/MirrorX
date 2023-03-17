<script lang="ts">
	import './files.css';
	import { onDestroy, onMount } from 'svelte';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { appWindow } from '@tauri-apps/api/window';
	import { invoke_config_language_get, invoke_config_theme_get } from '$lib/components/command';
	import { loadAllLocalesAsync } from '$lib/i18n/i18n-util.async';
	import { setLocale } from '$lib/i18n/i18n-svelte';
	import type { Locales } from '$lib/i18n/i18n-types';
	import { detectLocale, isLocale } from '$lib/i18n/i18n-util';
	import { navigatorDetector } from 'typesafe-i18n/detectors';
	import DialogNotificationFile from '$lib/widgets/dialog_notification_file.svelte';
	import { page } from '$app/stores';
	import { formatDeviceID } from '$lib/components/utility';
	import LL, { locale } from '$lib/i18n/i18n-svelte';

	let remote_device_id: string = $page.url.searchParams.get('device_id')!;

	let theme_change_unlisten_fn: UnlistenFn | null = null;
	let update_theme_unlisten_fn: UnlistenFn | null = null;
	let update_language_unlisten_fn: UnlistenFn | null = null;

	onMount(async () => {
		await loadAllLocalesAsync();

		let theme = await invoke_config_theme_get();
		if (theme && theme != 'auto') {
			document.getElementsByTagName('html').item(0)?.setAttribute('data-theme', theme);
		} else {
			let appTheme = await appWindow.theme();
			if (appTheme) {
				document.getElementsByTagName('html').item(0)?.setAttribute('data-theme', appTheme);
			} else {
				document.getElementsByTagName('html').item(0)?.setAttribute('data-theme', 'light');
			}
		}

		let language = await invoke_config_language_get();
		if (!isLocale(language)) {
			language = detectLocale(navigatorDetector);
		}
		setLocale(language as Locales);

		appWindow.setTitle(`MirrorX ${$LL.FileTransfer.WindowTitle()} ${remote_device_id}`);

		update_language_unlisten_fn = await listen<{ language: Locales }>(
			'update_language',
			async (event) => {
				console.log(event.payload);
				setLocale(event.payload.language);
				appWindow.setTitle(`MirrorX ${$LL.FileTransfer.WindowTitle()} ${remote_device_id}`);
			}
		);

		update_theme_unlisten_fn = await listen<'light' | 'dark' | 'auto'>(
			'update_theme',
			async (event) => {
				const theme = event.payload;
				if (theme == 'auto') {
					let appTheme = await appWindow.theme();
					if (appTheme) {
						document.getElementsByTagName('html').item(0)?.setAttribute('data-theme', appTheme);
					}
				} else {
					document.getElementsByTagName('html').item(0)?.setAttribute('data-theme', theme);
				}
			}
		);

		theme_change_unlisten_fn = await appWindow.onThemeChanged(async (event) => {
			let theme = await invoke_config_theme_get();
			if (theme == 'auto') {
				document.getElementsByTagName('html').item(0)?.setAttribute('data-theme', event.payload);
			}
		});
	});

	onDestroy(() => {
		if (theme_change_unlisten_fn) {
			theme_change_unlisten_fn();
		}

		if (update_language_unlisten_fn) {
			update_language_unlisten_fn();
		}

		if (update_language_unlisten_fn) {
			update_language_unlisten_fn();
		}
	});
</script>

<slot />
<DialogNotificationFile />
