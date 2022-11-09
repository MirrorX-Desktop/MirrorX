<script lang="ts">
	import SettingsNotificationCenter from './settings_notification_center.svelte';
	import { page } from '$app/stores';
	import LL, { setLocale } from '$lib/i18n/i18n-svelte';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import { locales } from '$lib/i18n/i18n-util';
	import type { Locales } from '$lib/i18n/i18n-types';
	import { WebviewWindow } from '@tauri-apps/api/window';

	let unlisten_fn: UnlistenFn | null = null;

	onMount(async () => {
		unlisten_fn = await listen<string>('global:update_language', (event) => {
			let language = event.payload;
			let language_locale = locales.find((v) => v === (language as Locales));
			if (language_locale) {
				const thisWindow = WebviewWindow.getByLabel('window_settings');
				setLocale(language_locale);
				thisWindow?.setTitle($LL.Settings.WindowTitle());
			} else {
				console.log(`unknown locale ${language}`);
			}
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
