<script lang="ts">
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import LL from '$lib/i18n/i18n-svelte';
	import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app';
	import icon from '../../src-tauri/assets/icons/icon.png';
	import { invoke_utility_enum_graphics_cards, invoke_utility_detect_os_platform } from '$lib/components/command';
	import UAParser from 'ua-parser-js';
	import { isMacOS } from '$lib/components/types';
	import { writeText } from '@tauri-apps/api/clipboard';
	import { appWindow } from '@tauri-apps/api/window';

	let show: boolean = false;
	let unlisten_fn: UnlistenFn | null = null;

	let ua = new UAParser();
	let name = '';
	let version = '';
	let tauri_version = '';
	let platform_info = '';
	let graphics_cards: Array<{ name: string; is_default: boolean }> = [];
	let webkit_version = `${ua.getBrowser().version} (${ua.getBrowser().name})`;

	onMount(async () => {
		unlisten_fn = await listen<void>('/dialog/about', async () => {
			const windowVisible = await appWindow.isVisible();
			if (!windowVisible) {
				await appWindow.show();
				await appWindow.unminimize();
			}

			show = true;
		});

		name = await getName();
		version = await getVersion();
		tauri_version = await getTauriVersion();
		platform_info = await invoke_utility_detect_os_platform();
		graphics_cards = await invoke_utility_enum_graphics_cards();

		console.log(new UAParser().getResult());
		console.log(navigator.userAgent);
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}
	});

	const copyToClipboard = async () => {
		let content = `Version: ${version}\n`;
		content += `Tauri Version: ${tauri_version}\n`;
		content += `OS: ${platform_info}\n`;
		content += `Webview: ${webkit_version}\n`;
		content += `GraphicsCards: ${graphics_cards.map((v) => v.name).join(',')}`;

		await writeText(content);
		show = false;
	};
</script>

<slot>
	<input type="checkbox" id="dialog_about" class="modal-toggle" checked={show} />
	<div data-tauri-drag-region class="modal {isMacOS ? '' : 'rounded-lg'}">
		<div class="modal-box">
			<h3 class="text-center text-lg font-bold">{name}</h3>
			<div class="flex flex-row items-center justify-center p-2">
				<img src={icon} width="48" class="text-center" alt="logo" />
			</div>
			<div class="flex select-auto flex-col items-center justify-center pb-2 text-sm">
				<div>{$LL.Dialogs.About.Version()}:&nbsp;{version}</div>
				<div>Tauri&nbsp;{$LL.Dialogs.About.Version()}:&nbsp;{tauri_version}</div>
				<div>OS:&nbsp;{platform_info}</div>
				<div>WebView:&nbsp;{webkit_version}</div>
				<div>Graphics Cards:</div>
				<div class="flex flex-col items-center">
					{#each graphics_cards as graphics_card}
						<div class="text-xs">{graphics_card.name}{graphics_card.is_default ? ' (Default)' : ''}</div>
					{/each}
				</div>
			</div>
			<div class="modal-action flex flex-row pt-2">
				<button class="btn flex-1" on:click={copyToClipboard}>{$LL.Dialogs.About.Copy()}</button>
				<button class="btn flex-1" on:click={() => (show = false)}>{$LL.DialogActions.Ok()}</button>
			</div>
		</div>
	</div>
</slot>
