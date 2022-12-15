<script lang="ts">
	import '../app.css';
	import { onDestroy, onMount } from 'svelte';
	import { loadAllLocalesAsync } from '$lib/i18n/i18n-util.async';
	import { loadAllLocales } from '$lib/i18n/i18n-util.sync';
	import Fa from 'svelte-fa';
	import {
		faNetworkWired,
		faClock,
		faSlidersH,
		faMinus,
		faXmark,
		faCircle,
		faCircleHalfStroke
	} from '@fortawesome/free-solid-svg-icons';
	import { faGithub } from '@fortawesome/free-brands-svg-icons';
	import org from '../../src-tauri/assets/icons/org.png';
	import { appWindow } from '@tauri-apps/api/window';
	import { current_domain } from '$lib/components/stores';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import type { UpdateLanguageEvent } from '$lib/components/rust_event';
	import type { Locales } from '$lib/i18n/i18n-types';
	import { setLocale } from '$lib/i18n/i18n-svelte';
	import * as commands from '$lib/components/command';
	import { emitNotification } from '$lib/components/notification';
	import { detectLocale, isLocale } from '$lib/i18n/i18n-util';
	import { get, type Unsubscriber } from 'svelte/store';
	import { navigatorDetector } from 'typesafe-i18n/detectors';
	import type { Domain } from '$lib/components/types';
	import { os } from '@tauri-apps/api';
	import { page } from '$app/stores';
	import Notification from '$lib/widgets/dialog_notification.svelte';
	import DialogNotification from '$lib/widgets/dialog_notification.svelte';
	import DialogVisitPrepare from '$lib/widgets/dialog_visit_prepare.svelte';

	let isMacOS: boolean = navigator.platform.toLowerCase().includes('mac');
	let domain: Domain | null = null;
	let domain_unsubscribe: Unsubscriber | null = null;
	let switch_primary_unlisten_fn: UnlistenFn | null = null;
	let update_language_unlisten_fn: UnlistenFn | null = null;

	loadAllLocales();

	onMount(async () => {
		if (import.meta.env.PROD) {
			document.addEventListener('contextmenu', (event) => event.preventDefault());
		}

		// isMacOS = (await os.type()) === 'Darwin';

		domain_unsubscribe = current_domain.subscribe((value) => {
			console.log('layout update domain');
			domain = value;
		});

		switch_primary_unlisten_fn = await listen('home:switch_primary_domain', switch_primary_domain);

		update_language_unlisten_fn = await listen<UpdateLanguageEvent>('update_language', (event) =>
			setLocale(event.payload.language as Locales)
		);

		try {
			await commands.invoke_config_init();
			console.log('finish init config');

			let language = await commands.invoke_config_language_get();
			if (isLocale(language)) {
				setLocale(language);
			} else {
				const detect_language = detectLocale(navigatorDetector);
				setLocale(detect_language);
				await commands.invoke_config_language_set(detect_language);
			}
			console.log('finish set locale');

			await commands.invoke_lan_init(false);
			console.log('finish init lan discover');

			current_domain.set(await commands.invoke_config_domain_get());
			console.log(`current domain: ${get(current_domain)}`);

			await commands.invoke_signaling_connect(false);
			console.log('finish init signaling');

			console.log('finish init current domain');
		} catch (error: any) {
			console.log(error);
			await emitNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	});

	onDestroy(() => {
		if (domain_unsubscribe) {
			domain_unsubscribe();
		}

		if (switch_primary_unlisten_fn) {
			switch_primary_unlisten_fn();
		}

		if (update_language_unlisten_fn) {
			update_language_unlisten_fn();
		}
	});

	const switch_primary_domain = async () => {
		try {
			await commands.invoke_signaling_connect(true);
		} catch (error: any) {
			await emitNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};

	const show_select_language_dialog = async () => {
		await emit('home:show_select_language_dialog');
	};

	const open_settings_window = () => {
		// const webview = new WebviewWindow('window_settings', {
		// 	url: '/settings/domain',
		// 	resizable: false,
		// 	maximized: false,
		// 	maxWidth: 680,
		// 	height: 580,
		// 	center: true,
		// 	title: $LL.Settings.WindowTitle()
		// });
		// since the webview window is created asynchronously,
		// Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
		// webview.once('tauri://created', function () {
		// 	// webview window successfully created
		// });
		// webview.once('tauri://error', function (e) {
		// 	// an error happened creating the webview window
		// });
	};
</script>

<div class="flex h-full bg-gray-100 {isMacOS ? 'flex-row' : 'flex-row-reverse rounded-lg border border-gray-600'}">
	<div data-tauri-drag-region class="absolute left-0 right-0 top-0 h-2" />

	{#if !isMacOS}
		<div data-tauri-drag-region class="titlebar gap-1">
			<button class="titlebar-button" id="titlebar-minimize" on:click={() => appWindow.minimize()}>
				<Fa icon={faMinus} size="xs" />
			</button>
			<button class="titlebar-button" id="titlebar-close" on:click={() => appWindow.hide()}>
				<Fa icon={faXmark} size="xs" />
			</button>
		</div>

		<div data-tauri-drag-region class="flex flex-col justify-between">
			<div data-tauri-drag-region class="navigation">
				<ul>
					<li
						class="navigation-item  p-1 {$page.url.pathname == '/home'
							? 'navigation-item-selected-right'
							: 'navigation-item-unselected'}"
					>
						<a href="/home" class="flex h-full w-full items-center justify-center hover:cursor-pointer">
							<img src={org} width="32" alt="main navigation tab" />
						</a>
					</li>
					<li
						class="navigation-item p-1 {$page.url.pathname == '/lan'
							? 'navigation-item-selected-right'
							: 'navigation-item-unselected'}"
					>
						<a href="/lan" class="flex h-full w-full items-center justify-center hover:cursor-pointer">
							<Fa icon={faNetworkWired} />
						</a>
					</li>
					<li
						class="navigation-item p-1 {$page.url.pathname == '/history'
							? 'navigation-item-selected-right'
							: 'navigation-item-unselected'}"
					>
						<a href="/history" class="flex h-full w-full items-center justify-center hover:cursor-pointer">
							<Fa icon={faClock} />
						</a>
					</li>
					<li
						class="navigation-item p-1 {$page.url.pathname == '/settings'
							? 'navigation-item-selected-right'
							: 'navigation-item-unselected'}"
					>
						<a href="/settings" class="flex h-full w-full items-center justify-center hover:cursor-pointer">
							<Fa icon={faSlidersH} />
						</a>
					</li>
					<div class="navigation-indicator-right" />
				</ul>
			</div>
			<div class="flex flex-col items-center pb-2">
				<div class="h-12 w-12 p-2">
					<label class="swap swap-rotate navigation-extra-item h-full w-full rounded-lg">
						<input type="checkbox" />

						<!-- sun icon -->
						<svg class="swap-on h-4 w-4 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
							><path
								d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"
							/></svg
						>

						<!-- moon icon -->
						<svg class="swap-off h-4 w-4 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
							><path
								d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"
							/></svg
						>
					</label>
				</div>
				<div class="h-12 w-12 p-2">
					<a
						href="https://github.com/MirrorX-Desktop/MirrorX"
						rel="noreferrer"
						target="_blank"
						class="navigation-extra-item flex h-full w-full items-center justify-center rounded-lg"
					>
						<Fa icon={faGithub} />
					</a>
				</div>
			</div>
		</div>

		<div class="h-full w-full flex-1 py-2 pl-2">
			<div class="content">
				<slot />
			</div>
		</div>
	{/if}

	{#if isMacOS}
		<div data-tauri-drag-region class="flex flex-col justify-between">
			<div data-tauri-drag-region class="navigation">
				<ul>
					<li
						class="navigation-item  p-1 {$page.url.pathname == '/home'
							? 'navigation-item-selected'
							: 'navigation-item-unselected'}"
					>
						<a href="/home" class="flex h-full w-full items-center justify-center hover:cursor-pointer">
							<img src={org} width="32" alt="main navigation tab" />
						</a>
					</li>
					<li
						class="navigation-item p-1 {$page.url.pathname == '/lan'
							? 'navigation-item-selected'
							: 'navigation-item-unselected'}"
					>
						<a href="/lan" class="flex h-full w-full items-center justify-center hover:cursor-pointer">
							<Fa icon={faNetworkWired} />
						</a>
					</li>
					<li
						class="navigation-item p-1 {$page.url.pathname == '/history'
							? 'navigation-item-selected'
							: 'navigation-item-unselected'}"
					>
						<a href="/history" class="flex h-full w-full items-center justify-center hover:cursor-pointer">
							<Fa icon={faClock} />
						</a>
					</li>
					<li
						class="navigation-item p-1 {$page.url.pathname == '/settings'
							? 'navigation-item-selected'
							: 'navigation-item-unselected'}"
					>
						<a href="/settings" class="flex h-full w-full items-center justify-center hover:cursor-pointer">
							<Fa icon={faSlidersH} />
						</a>
					</li>
					<div class="navigation-indicator" />
				</ul>
			</div>
			<div class="flex flex-col items-center pb-2">
				<div class="h-12 w-12 p-2">
					<label class="swap swap-rotate navigation-extra-item h-full w-full rounded-lg">
						<input type="checkbox" />

						<!-- sun icon -->
						<svg class="swap-on h-4 w-4 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
							><path
								d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"
							/></svg
						>

						<!-- moon icon -->
						<svg class="swap-off h-4 w-4 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
							><path
								d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"
							/></svg
						>
					</label>
				</div>
				<div class="h-12 w-12 p-2">
					<a
						href="https://github.com/MirrorX-Desktop/MirrorX"
						rel="noreferrer"
						target="_blank"
						class="navigation-extra-item flex h-full w-full items-center justify-center rounded-lg"
					>
						<Fa icon={faGithub} />
					</a>
				</div>
			</div>
		</div>

		<div class="h-full w-full flex-1 py-2 pr-2">
			<div class="content">
				<slot />
			</div>
		</div>
	{/if}
</div>

<DialogNotification />
<DialogVisitPrepare />

<style>
	:root {
		--navigation-top-offset: 28px;
	}

	.titlebar {
		width: 48px;
		height: var(--navigation-top-offset);
		z-index: 5;
		user-select: none;
		display: flex;
		justify-content: center;
		align-items: center;
		position: absolute;
		right: 0;
		top: 0;
	}

	.titlebar-button {
		border-radius: 4px;
		display: inline-flex;
		justify-content: center;
		align-items: center;
		width: 16px;
		height: 16px;
		transition: color 100ms linear;
		transition: background-color 100ms linear;
	}

	.titlebar-button:hover {
		color: white;
		background-color: white;
	}

	#titlebar-minimize:hover {
		color: white;
		background-color: rgb(185, 185, 185);
	}

	#titlebar-close:hover {
		color: white;
		background-color: #bb3333;
	}

	.navigation {
		padding-top: var(--navigation-top-offset);
		width: 48px;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.navigation ul li {
		position: relative;
		list-style: none;
		width: 48px;
		height: 48px;
		z-index: 2;
	}

	.navigation-item-selected {
		@apply text-primary;
		transition: 0.3s;
		transform: translateX(4px);
	}

	.navigation-item-selected-right {
		@apply text-primary;
		transition: 0.3s;
		transform: translateX(-4px);
	}

	.navigation-item-unselected {
		color: var(--tw-primary);
		transition: 0.3s;
	}

	.navigation-indicator {
		background-color: white;
		position: absolute;
		top: var(--navigation-top-offset);
		left: 4px;
		width: 45px;
		height: 48px;
		border-top-left-radius: 8px;
		border-bottom-left-radius: 8px;
		transition: 0.3s;
		box-shadow: 0px 0px 16px rgba(198, 198, 198, 0.729);
		clip-path: inset(-16px 0px -16px -16px);
		z-index: 1;
	}

	.navigation-indicator-right {
		background-color: white;
		position: absolute;
		top: var(--navigation-top-offset);
		right: 4px;
		width: 45px;
		height: 48px;
		border-top-right-radius: 8px;
		border-bottom-right-radius: 8px;
		transition: 0.3s;
		box-shadow: 0px 0px 16px rgba(198, 198, 198, 0.729);
		clip-path: inset(-16px -16px -16px 0px);
		z-index: 1;
	}

	.navigation-indicator::before,
	.navigation-indicator::after {
		content: '';
		position: absolute;
		right: 0px;
		width: 16px;
		height: 16px;
		background-color: transparent;
		border-radius: 100%;
		z-index: 2;
	}

	.navigation-indicator-right::before,
	.navigation-indicator-right::after {
		content: '';
		position: absolute;
		left: 0px;
		width: 16px;
		height: 16px;
		background-color: transparent;
		border-radius: 100%;
		z-index: 2;
	}

	.navigation-indicator::before {
		top: -16px;
		box-shadow: 8px 8px white;
	}

	.navigation-indicator-right::before {
		top: -16px;
		box-shadow: -8px 8px white;
	}

	.navigation-indicator::after {
		bottom: -16px;
		box-shadow: 8px -8px white;
	}

	.navigation-indicator-right::after {
		bottom: -16px;
		box-shadow: -8px -8px white;
	}

	.navigation ul li:nth-child(1).navigation-item-selected ~ .navigation-indicator,
	.navigation ul li:nth-child(1).navigation-item-selected-right ~ .navigation-indicator-right {
		transform: translateY(calc(48px * 0));
	}

	.navigation ul li:nth-child(2).navigation-item-selected ~ .navigation-indicator,
	.navigation ul li:nth-child(2).navigation-item-selected-right ~ .navigation-indicator-right {
		transform: translateY(calc(48px * 1));
	}

	.navigation ul li:nth-child(3).navigation-item-selected ~ .navigation-indicator,
	.navigation ul li:nth-child(3).navigation-item-selected-right ~ .navigation-indicator-right {
		transform: translateY(calc(48px * 2));
	}

	.navigation ul li:nth-child(4).navigation-item-selected ~ .navigation-indicator,
	.navigation ul li:nth-child(4).navigation-item-selected-right ~ .navigation-indicator-right {
		transform: translateY(calc(48px * 3));
	}

	.navigation-extra-item {
		transition: 0.3s;
	}

	.navigation-extra-item:hover {
		cursor: pointer;
		transition: 0.3s;
		border-radius: 8px;
		box-shadow: 0px 0px 16px rgba(198, 198, 198, 0.729);
	}

	.content {
		width: 100%;
		height: 100%;
		border-radius: 8px;
		background: white;
		box-shadow: 0px 0px 16px rgba(198, 198, 198, 0.729);
		z-index: 2;
		flex: 1 1 0%;
	}
</style>
