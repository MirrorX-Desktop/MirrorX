<script lang="ts">
	import './main.css';
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
		faCircleHalfStroke,
		faLanguage
	} from '@fortawesome/free-solid-svg-icons';
	import { faGithub } from '@fortawesome/free-brands-svg-icons';
	import logoLight from '$lib/../src-tauri/assets/icons/org.png';
	import logoDark from '$lib/../src-tauri/assets/icons/tray-macOS.png';
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
	import { hide } from '@tauri-apps/api/app';
	import DialogAbout from '$lib/widgets/dialog_about.svelte';
	import DialogLanConnect from '$lib/widgets/dialog_lan_connect.svelte';
	import DialogSelectLanguage from '$lib/widgets/dialog_select_language.svelte';
	import DialogDomainList from '$lib/widgets/dialog_domain_list.svelte';
	import DialogDomainAdd from '$lib/widgets/dialog_domain_add.svelte';
	import DialogDomainDelete from '$lib/widgets/dialog_domain_delete.svelte';
	import DialogDomainEdit from '$lib/widgets/dialog_domain_edit.svelte';
	import DialogDomainSwitch from '$lib/widgets/dialog_domain_switch.svelte';
	import DialogHistoryConnect from '$lib/widgets/dialog_history_connect.svelte';

	const observer = new MutationObserver((mutations: MutationRecord[], observer: MutationObserver) => {
		for (const mutation of mutations) {
			if (mutation.type === 'attributes') {
				if (mutation.attributeName == 'data-theme') {
					let node = mutation.target as HTMLElement;
					let themeValue = node.getAttribute('data-theme');
					if (themeValue) currentTheme = themeValue as 'light' | 'dark';
					return;
				}
			}
		}
	});

	let isMacOS: boolean = navigator.platform.toLowerCase().includes('mac');
	let theme_change_unlisten_fn: UnlistenFn | null = null;
	let currentTheme: 'light' | 'dark';

	onMount(async () => {
		let htmlNode = document.getElementsByTagName('html').item(0);
		if (htmlNode) {
			observer.observe(htmlNode, { attributes: true });
		}

		if (import.meta.env.PROD) {
			document.addEventListener('contextmenu', (event) => event.preventDefault());
		}

		await loadAllLocalesAsync();

		theme_change_unlisten_fn = await appWindow.onThemeChanged(async (event) => {
			let theme = await commands.invoke_config_theme_get();
			if (theme == 'auto') {
				document.getElementsByTagName('html').item(0)?.setAttribute('data-theme', event.payload);
			}
		});

		try {
			// only has effect with macOS, other platform is no-op
			await commands.invoke_utility_hide_macos_zoom_button();

			await commands.invoke_config_init();
			console.log('finish init config');

			let theme = await commands.invoke_config_theme_get();
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

			let language = await commands.invoke_config_language_get();
			if (!isLocale(language)) {
				language = detectLocale(navigatorDetector);
			}
			setLocale(language as Locales);
			// always set language to make sure system tray menu use correct language
			await commands.invoke_config_language_set(language);
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
		if (theme_change_unlisten_fn) {
			theme_change_unlisten_fn();
		}

		observer.disconnect();
	});

	const show_select_language_dialog = async () => {
		await emit('/dialog/select_language');
	};
</script>

<div
	class="bg-base-100 flex h-full transition-all {isMacOS
		? 'flex-row'
		: 'flex-row-reverse rounded-lg border border-gray-600'}"
>
	<div data-tauri-drag-region class="absolute left-0 right-0 top-0 h-2" />

	{#if !isMacOS}
		<div data-tauri-drag-region class="titlebar gap-1">
			<button class="titlebar-button" id="titlebar-minimize" on:click={async () => await appWindow.minimize()}>
				<Fa icon={faMinus} size="xs" />
			</button>
			<button class="titlebar-button" id="titlebar-close" on:click={async () => await appWindow.hide()}>
				<Fa icon={faXmark} size="xs" />
			</button>
		</div>
	{/if}

	<div data-tauri-drag-region class="flex flex-col justify-between">
		<div data-tauri-drag-region class="navigation">
			<ul>
				<li
					class="navigation-item p-1 {$page.url.pathname == '/main/home'
						? isMacOS
							? 'navigation-item-selected'
							: 'navigation-item-selected-right'
						: 'navigation-item-unselected'}"
				>
					<a href="/main/home" class="flex h-full w-full items-center justify-center hover:cursor-pointer">
						{#if currentTheme == 'light'}
							<img src={logoLight} width="32" alt="main navigation tab" />
						{:else}
							<img src={logoDark} width="32" alt="main navigation tab" />
						{/if}
					</a>
				</li>
				<li
					class="navigation-item p-1 {$page.url.pathname == '/main/lan'
						? isMacOS
							? 'navigation-item-selected'
							: 'navigation-item-selected-right'
						: 'navigation-item-unselected'}"
				>
					<a href="/main/lan" class="flex h-full w-full items-center justify-center hover:cursor-pointer">
						<Fa icon={faNetworkWired} />
					</a>
				</li>
				<li
					class="navigation-item p-1 {$page.url.pathname == '/main/history'
						? isMacOS
							? 'navigation-item-selected'
							: 'navigation-item-selected-right'
						: 'navigation-item-unselected'}"
				>
					<a href="/main/history" class="flex h-full w-full items-center justify-center hover:cursor-pointer">
						<Fa icon={faClock} />
					</a>
				</li>
				<li
					class="navigation-item p-1 {$page.url.pathname == '/main/settings'
						? isMacOS
							? 'navigation-item-selected'
							: 'navigation-item-selected-right'
						: 'navigation-item-unselected'}"
				>
					<a href="/main/settings" class="flex h-full w-full items-center justify-center hover:cursor-pointer">
						<Fa icon={faSlidersH} />
					</a>
				</li>
				<div class={isMacOS ? 'navigation-indicator' : 'navigation-indicator-right'} />
			</ul>
		</div>
		<div class="flex flex-col items-center pb-2">
			<div class="h-12 w-12 p-2">
				<button
					class="navigation-extra-item flex h-full w-full items-center justify-center rounded-lg"
					on:click={show_select_language_dialog}
				>
					<Fa icon={faLanguage} />
				</button>
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

	<div class="h-full w-full flex-1 py-2 {isMacOS ? 'pr-2' : 'pl-2'}">
		<div class="content">
			<slot />
		</div>
	</div>
</div>

<DialogAbout />
<DialogNotification />
<DialogVisitPrepare />
<DialogLanConnect />
<DialogSelectLanguage />
<DialogDomainList />
<DialogDomainAdd />
<DialogDomainEdit />
<DialogDomainSwitch />
<DialogDomainDelete />
<DialogHistoryConnect />

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

	#titlebar-minimize:hover {
		@apply bg-base-300;
		color: white;
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
		@apply text-primary duration-300;
		transform: translateX(4px);
	}

	.navigation-item-selected-right {
		@apply text-primary duration-300;
		transform: translateX(-4px);
	}

	.navigation-item-unselected {
		@apply duration-300;
	}

	.navigation-indicator {
		@apply shadow-base-300 bg-base-100 duration-300;
		position: absolute;
		top: var(--navigation-top-offset);
		left: 4px;
		width: 45px;
		height: 48px;
		border-top-left-radius: 8px;
		border-bottom-left-radius: 8px;
		box-shadow: 0px 0px 16px var(--tw-shadow-color);
		clip-path: inset(-16px 0px -16px -16px);
		z-index: 1;
	}

	.navigation-indicator-right {
		@apply shadow-base-300 bg-base-100 duration-300;
		position: absolute;
		top: var(--navigation-top-offset);
		right: 4px;
		width: 45px;
		height: 48px;
		border-top-right-radius: 8px;
		border-bottom-right-radius: 8px;
		box-shadow: 0px 0px 16px var(--tw-shadow-color);
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
		@apply shadow-base-100 duration-300;
		top: -16px;
		box-shadow: 8px 8px var(--tw-shadow-color);
	}

	.navigation-indicator-right::before {
		@apply shadow-base-100 duration-300;
		top: -16px;
		box-shadow: -8px 8px var(--tw-shadow-color);
	}

	.navigation-indicator::after {
		@apply shadow-base-100 duration-300;
		bottom: -16px;
		box-shadow: 8px -8px var(--tw-shadow-color);
	}

	.navigation-indicator-right::after {
		@apply shadow-base-100 duration-300;
		bottom: -16px;
		box-shadow: -8px -8px var(--tw-shadow-color);
	}

	.navigation ul li:nth-child(1).navigation-item-selected ~ .navigation-indicator,
	.navigation ul li:nth-child(1).navigation-item-selected-right ~ .navigation-indicator-right {
		@apply duration-300;
		transform: translateY(calc(48px * 0));
	}

	.navigation ul li:nth-child(2).navigation-item-selected ~ .navigation-indicator,
	.navigation ul li:nth-child(2).navigation-item-selected-right ~ .navigation-indicator-right {
		@apply duration-300;
		transform: translateY(calc(48px * 1));
	}

	.navigation ul li:nth-child(3).navigation-item-selected ~ .navigation-indicator,
	.navigation ul li:nth-child(3).navigation-item-selected-right ~ .navigation-indicator-right {
		@apply duration-300;
		transform: translateY(calc(48px * 2));
	}

	.navigation ul li:nth-child(4).navigation-item-selected ~ .navigation-indicator,
	.navigation ul li:nth-child(4).navigation-item-selected-right ~ .navigation-indicator-right {
		@apply duration-300;
		transform: translateY(calc(48px * 3));
	}

	.navigation-extra-item {
		@apply duration-300;
	}

	.navigation-extra-item:hover {
		@apply shadow-base-300;
		cursor: pointer;
		border-radius: 8px;
		box-shadow: 0px 0px 16px var(--tw-shadow-color);
	}

	.content {
		@apply shadow-base-300 bg-base-100 duration-300;
		width: 100%;
		height: 100%;
		border-radius: 8px;
		box-shadow: 0px 0px 16px var(--tw-shadow-color);
		z-index: 2;
		flex: 1 1 0%;
	}
</style>
