<script lang="ts">
	import SettingsNotificationCenter from './settings_notification_center.svelte';
	import { page } from '$app/stores';
	import LL, { setLocale } from '$lib/i18n/i18n-svelte';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import type { Locales } from '$lib/i18n/i18n-types';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import type { UpdateLanguageEvent } from '$lib/components/rust_event';
	import { invoke_config_language_get, invoke_config_language_set } from '$lib/components/command';
	import { detectLocale, isLocale } from '$lib/i18n/i18n-util';
	import { navigatorDetector } from 'typesafe-i18n/detectors';

	let unlisten_fn: UnlistenFn | null = null;

	onMount(async () => {
		let language = await invoke_config_language_get();
		if (isLocale(language)) {
			setLocale(language);
		} else {
			const detect_language = detectLocale(navigatorDetector);
			setLocale(detect_language);
			await invoke_config_language_set(detect_language);
		}

		unlisten_fn = await listen<UpdateLanguageEvent>('update_language', (event) => {
			setLocale(event.payload.language as Locales);
			const thisWindow = WebviewWindow.getByLabel('window_settings');
			thisWindow?.setTitle($LL.Settings.WindowTitle());
		});
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}
	});
</script>

<div class="flex h-full w-full flex-row ">
	<div class="h-full border-r">
		<ul class="menu bg-base-100 w-56">
			<li>
				<a href="/settings/domain" class={$page.url.pathname == '/settings/domain' ? 'active' : ''}>
					{$LL.Settings.Layout.Domain()}
				</a>
			</li>
			<li>
				<a href="/settings/about" class={$page.url.pathname == '/settings/about' ? 'active' : ''}>
					{$LL.Settings.Layout.About()}
				</a>
			</li>
		</ul>
	</div>

	<div class="h-full flex-1">
		<slot />
	</div>
</div>

<SettingsNotificationCenter />
