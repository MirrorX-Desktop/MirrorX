<script lang="ts">
	import { invoke_config_theme_get, invoke_config_theme_set } from '$lib/components/command';
	import LL from '$lib/i18n/i18n-svelte';
	import { faSun, faMoon, faCircleHalfStroke } from '@fortawesome/free-solid-svg-icons';
	import { appWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import Fa from 'svelte-fa';

	let currentTheme: 'light' | 'dark' | 'auto';

	onMount(async () => {
		currentTheme =
			(await invoke_config_theme_get()) ??
			(document.getElementsByTagName('html').item(0)?.getAttribute('data-theme') as 'light' | 'dark') ??
			'light';
		await invoke_config_theme_set(currentTheme);
	});

	const changeTheme = async (theme: 'light' | 'dark' | 'auto') => {
		if (theme == 'auto') {
			let appTheme = await appWindow.theme();
			if (appTheme) {
				document.getElementsByTagName('html').item(0)?.setAttribute('data-theme', appTheme);
			}
		} else {
			document.getElementsByTagName('html').item(0)?.setAttribute('data-theme', theme);
		}

		currentTheme = theme;
		await invoke_config_theme_set(currentTheme);
	};
</script>

<slot>
	<div class="divider mt-0">{$LL.Settings.Appearance.Title()}</div>
	<div class="flex w-full flex-row items-center justify-between">
		<div class="flex-1">{$LL.Settings.Appearance.Theme()}</div>
		<div class="flex flex-row items-center justify-end gap-2">
			<button
				class="hover:bg-base-200 flex cursor-pointer flex-col items-center gap-1 rounded-lg p-2 {currentTheme == 'light'
					? 'ring-info ring'
					: ''}"
				on:click={() => changeTheme('light')}
			>
				<Fa icon={faSun} />
				<div class="text-sm">{$LL.Settings.Appearance.Light()}</div>
			</button>
			<button
				class="hover:bg-base-200 flex cursor-pointer flex-col items-center gap-1 rounded-lg p-2 {currentTheme == 'dark'
					? 'ring-info ring'
					: ''}"
				on:click={() => changeTheme('dark')}
			>
				<Fa icon={faMoon} />
				<div class="text-sm">{$LL.Settings.Appearance.Dark()}</div>
			</button>
			<button
				class="hover:bg-base-200 flex cursor-pointer flex-col items-center gap-1 rounded-lg p-2 {currentTheme == 'auto'
					? 'ring-info ring'
					: ''}"
				on:click={() => changeTheme('auto')}
			>
				<Fa icon={faCircleHalfStroke} />
				<div class="text-sm">{$LL.Settings.Appearance.Auto()}</div>
			</button>
		</div>
	</div>
</slot>
