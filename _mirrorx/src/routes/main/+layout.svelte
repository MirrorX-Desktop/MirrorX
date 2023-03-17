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
		faLanguage,
		faThumbTack
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
	import { stringify } from 'postcss';

	const observer = new MutationObserver(
		(mutations: MutationRecord[], observer: MutationObserver) => {
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
		}
	);

	let isMacOS: boolean = navigator.platform.toLowerCase().includes('mac');
	let theme_change_unlisten_fn: UnlistenFn | null = null;
	let currentTheme: 'light' | 'dark';
	let window_always_on_top: boolean = false;

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
			if (isMacOS) {
				await commands.invoke_utility_hide_macos_zoom_button();
			}

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

			await commands.invoke_portal_switch(false);

			current_domain.set(await commands.invoke_config_domain_get());
			console.log(`current domain: ${get(current_domain)}`);

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

	const switch_always_on_top = async () => {
		window_always_on_top = !window_always_on_top;
		await appWindow.setAlwaysOnTop(window_always_on_top);
	};

	function navigation_button_style(path: string): string {
		return $page.url.pathname == path ? 'navigation-item-selected' : 'navigation-item-unselected';
	}
</script>

<div class="bg-base-100 flex h-full flex-row transition-all">
	<div class="flex flex-col justify-center">
		<div class="navigation">
			<ul>
				<li
					class="navigation-item p-1 {$page.url.pathname == '/main/home'
						? 'navigation-item-selected'
						: 'navigation-item-unselected'}"
				>
					<a
						href="/main/home"
						class="tooltip tooltip-right flex h-full w-full items-center justify-center hover:cursor-pointer"
						data-tip="Devices"
					>
						{#if currentTheme == 'light'}
							<img src={logoLight} width="32" alt="main navigation tab" />
						{:else}
							<img src={logoDark} width="32" alt="main navigation tab" />
						{/if}
					</a>
				</li>
				<li
					class="navigation-item p-1 {$page.url.pathname == '/main/lan'
						? 'navigation-item-selected'
						: 'navigation-item-unselected'}"
				>
					<a
						href="/main/lan"
						class="tooltip tooltip-right flex h-full w-full items-center justify-center hover:cursor-pointer"
						data-tip="LAN Discovery"
					>
						<Fa icon={faNetworkWired} />
					</a>
				</li>
				<li
					class="navigation-item p-1 {$page.url.pathname == '/main/history'
						? 'navigation-item-selected'
						: 'navigation-item-unselected'}"
				>
					<a
						href="/main/history"
						class="tooltip tooltip-right flex h-full w-full items-center justify-center hover:cursor-pointer"
						data-tip="History"
					>
						<Fa icon={faClock} />
					</a>
				</li>
				<li
					class="navigation-item p-1 {$page.url.pathname == '/main/settings'
						? 'navigation-item-selected'
						: 'navigation-item-unselected'}"
				>
					<a
						href="/main/settings"
						class="tooltip tooltip-right flex h-full w-full items-center justify-center hover:cursor-pointer"
						data-tip="Settings"
					>
						<Fa icon={faSlidersH} />
					</a>
				</li>
				<div class="navigation-indicator" />
			</ul>
		</div>
		<!-- <div class="flex flex-col items-center">
			<div class="h-12 w-12 p-2">
				<button
					class="navigation-extra-item flex h-full w-full items-center justify-center rounded-lg"
					on:click={switch_always_on_top}
				>
					<Fa icon={faThumbTack} rotate={window_always_on_top ? 0 : 45} />
				</button>
			</div>
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
		</div> -->
	</div>

	<div class="h-full w-full flex-1">
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
	.navigation {
		position: relative;
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
		@apply bg-base-100 shadow-base-300 duration-300;
		position: absolute;
		top: 0px;
		left: 4px;
		width: 45px;
		height: 48px;
		border-top-left-radius: 8px;
		border-bottom-left-radius: 8px;
		box-shadow: 0px 0px 16px var(--tw-shadow-color);
		clip-path: inset(-16px 0px -16px -16px);
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

	.navigation-indicator::before {
		@apply shadow-base-100 duration-300;
		top: -16px;
		box-shadow: 8px 8px var(--tw-shadow-color);
	}

	.navigation-indicator::after {
		@apply shadow-base-100 duration-300;
		bottom: -16px;
		box-shadow: 8px -8px var(--tw-shadow-color);
	}

	.navigation ul li:nth-child(1).navigation-item-selected ~ .navigation-indicator {
		@apply duration-300;
		transform: translateY(calc(48px * 0));
	}

	.navigation ul li:nth-child(2).navigation-item-selected ~ .navigation-indicator {
		@apply duration-300;
		transform: translateY(calc(48px * 1));
	}

	.navigation ul li:nth-child(3).navigation-item-selected ~ .navigation-indicator {
		@apply duration-300;
		transform: translateY(calc(48px * 2));
	}

	.navigation ul li:nth-child(4).navigation-item-selected ~ .navigation-indicator {
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
		@apply bg-base-100 shadow-base-300 duration-300;
		width: 100%;
		height: 100%;
		box-shadow: 0px 0px 16px var(--tw-shadow-color);
		z-index: 2;
		flex: 1 1 0%;
	}
</style>
