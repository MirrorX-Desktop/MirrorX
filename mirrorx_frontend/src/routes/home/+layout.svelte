<script lang="ts">
	import { faSpinner, faSliders, faGear, faLanguage } from '@fortawesome/free-solid-svg-icons';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import LL, { setLocale } from '../../i18n/i18n-svelte';
	import DialogInputRemotePassword from './connect/dialog_input_remote_password.svelte';
	import DialogVisitRequest from './connect/dialog_visit_request.svelte';
	import HomeNotificationCenter, { emitHomeNotification } from './home_notification_center.svelte';
	import type { Unsubscriber } from 'svelte/store';
	import { page } from '$app/stores';
	import { current_domain, current_lan_discover_nodes, type CurrentDomain } from '../../components/stores';
	import {
		invoke_get_current_domain,
		invoke_get_language,
		invoke_get_lan_discover_nodes,
		invoke_init_config,
		invoke_init_lan,
		invoke_init_signaling
	} from '../../components/command';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import DialogSelectLanguage from './dialog_select_language.svelte';
	import type { UpdateLanguageEvent } from '$lib/components/rust_event';
	import type { Locales } from '$lib/i18n/i18n-types';

	let domain: CurrentDomain | null = null;
	let domain_unsubscribe: Unsubscriber | null = null;
	let switch_primary_unlisten_fn: UnlistenFn | null = null;
	let update_language_unlisten_fn: UnlistenFn | null = null;
	let update_lan_discover_nodes_unlisten_fn: UnlistenFn | null = null;

	onMount(async () => {
		domain_unsubscribe = current_domain.subscribe((value) => {
			console.log('layout update domain');
			domain = value;
		});

		switch_primary_unlisten_fn = await listen('home:switch_primary_domain', switch_primary_domain);

		update_language_unlisten_fn = await listen<UpdateLanguageEvent>('update_language', (event) =>
			setLocale(event.payload.language as Locales)
		);

		update_lan_discover_nodes_unlisten_fn = await listen<void>('update_lan_discover_nodes', async (_) => {
			try {
				let nodes = await invoke_get_lan_discover_nodes();
				current_lan_discover_nodes.set(nodes);
			} catch (error: any) {
				await emitHomeNotification({ level: 'error', title: 'Error', message: error.toString() });
			}
		});
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

		if (update_lan_discover_nodes_unlisten_fn != null) {
			update_lan_discover_nodes_unlisten_fn();
		}
	});

	(async function () {
		try {
			await invoke_init_config();
			console.log('finish init config');

			await invoke_init_lan({ force: false });
			console.log('finish init lan discover');

			await invoke_init_signaling({ force: false });
			console.log('finish init signaling');

			setLocale((await invoke_get_language()) as Locales);
			console.log('finish set locale');

			current_domain.set(await invoke_get_current_domain());
			console.log('finish init current domain');
		} catch (error: any) {
			console.log(error);
			await emitHomeNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	})();

	const switch_primary_domain = async () => {
		try {
			current_domain.set(await invoke_get_current_domain());
			await invoke_init_signaling({ force: true });
		} catch (error: any) {
			await emitHomeNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};

	const show_select_language_dialog = async () => {
		await emit('home:show_select_language_dialog');
	};

	const open_settings_window = () => {
		const webview = new WebviewWindow('window_settings', {
			url: '/settings/domain',
			resizable: false,
			maximized: false,
			maxWidth: 680,
			height: 580,
			center: true,

			title: $LL.Settings.WindowTitle()
		});
		// since the webview window is created asynchronously,
		// Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
		webview.once('tauri://created', function () {
			// webview window successfully created
		});
		webview.once('tauri://error', function (e) {
			// an error happened creating the webview window
		});
	};
</script>

<div data-tauri-drag-region class="flex h-full flex-col">
	<div data-tauri-drag-region class="mx-2 flex flex-none flex-col">
		<div data-tauri-drag-region class=" z-10 mt-2 mb-2 flex flex-row items-center justify-between">
			<button class="btn btn-xs invisible"><Fa icon={faSliders} /></button>
			<div class="text-2xl">{$LL.Home.Layout.Domain()}</div>

			<div class="dropdown dropdown-end">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<label tabindex="0" class="btn btn-xs"><Fa icon={faSliders} /></label>

				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box w-52 p-2 ring-1 ring-gray-300">
					<li>
						<button on:mouseup={open_settings_window}>
							<Fa class="h-5 w-5" icon={faGear} />
							{$LL.Home.Layout.Menu.Settings()}
						</button>
					</li>

					<li>
						<button on:mouseup={show_select_language_dialog}>
							<Fa class="h-5 w-5" icon={faLanguage} />
							{$LL.Home.Layout.Menu.Language()}
						</button>
					</li>
				</ul>
			</div>
		</div>
	</div>

	<div class="mx-2 flex flex-1 flex-col overflow-hidden">
		<div class="flex-none">
			<div class="my-2 text-center text-4xl">
				{#if domain}
					{domain.name}
				{:else}
					<Fa class="w-full text-center" icon={faSpinner} spin={true} size={'sm'} />
				{/if}
			</div>
			<div class="btn-group my-3 flex flex-row">
				<a href="/home/connect" class="btn flex-1 {$page.url.pathname == '/home/connect' ? 'btn-active' : undefined}">
					<svg class="h-5 w-5" fill="white" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"><path d="M920 496H768V232.1a40 40 0 0 0-40-40H512V104a40 40 0 0 0-40-40H104a40 40 0 0 0-40 40v240a40 40 0 0 0 40 40h368a40 40 0 0 0 40-40v-87.9h192V496H552a40 40 0 0 0-40 40v104H320V528h74a4 4 0 0 0 4-4v-64a4 4 0 0 0-4-4H182a4 4 0 0 0-4 4v64a4 4 0 0 0 4 4h74v136a40 40 0 0 0 40 40h216v72a40 40 0 0 0 40 40h368a40 40 0 0 0 40-40V536a40 40 0 0 0-40-40zM440 312H136V136h304z m448 432H584V568h304z m-46 144H630a4 4 0 0 0-4 4v64a4 4 0 0 0 4 4h212a4 4 0 0 0 4-4v-64a4 4 0 0 0-4-4z"></path></svg>
					&nbsp;{$LL.Home.Layout.Connect()}
				</a>
				<a href="/home/lan" class="btn flex-1 {$page.url.pathname == '/home/lan' ? 'btn-active' : undefined}">
					<svg class="h-5 w-5" fill="white" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"><path d="M512 300.8C396.8 300.8 307.2 396.8 307.2 512c0 57.6 25.6 115.2 70.4 153.6 12.8 12.8 32 12.8 44.8 0 12.8-12.8 12.8-32 0-44.8C384 595.2 364.8 556.8 364.8 512c0-83.2 64-147.2 147.2-147.2 83.2 0 147.2 64 147.2 147.2 0 83.2-64 147.2-147.2 147.2L505.6 659.2c-12.8 0-25.6 6.4-25.6 19.2-6.4 19.2-19.2 38.4-32 51.2-12.8 12.8-12.8 32 0 44.8 6.4 6.4 12.8 6.4 19.2 6.4 6.4 0 12.8 0 19.2-6.4 19.2-19.2 32-38.4 38.4-57.6 108.8-6.4 198.4-96 198.4-211.2C723.2 396.8 627.2 300.8 512 300.8zM121.6 512c0-115.2 38.4-224 115.2-307.2 12.8-12.8 12.8-32 0-44.8-12.8-12.8-32-12.8-44.8 0C108.8 262.4 64 384 64 512c0 128 44.8 249.6 134.4 345.6 6.4 6.4 12.8 12.8 25.6 12.8 6.4 0 12.8 0 19.2-6.4 12.8-12.8 12.8-32 0-44.8C166.4 736 121.6 627.2 121.6 512zM825.6 166.4c-12.8-12.8-32-12.8-44.8 0-12.8 12.8-12.8 32 0 44.8 76.8 83.2 115.2 198.4 115.2 307.2 0 115.2-38.4 224-115.2 307.2-12.8 12.8-12.8 32 0 44.8 6.4 6.4 12.8 6.4 19.2 6.4 6.4 0 19.2-6.4 25.6-12.8C915.2 761.6 960 640 960 512 960 384 915.2 262.4 825.6 166.4zM288 307.2C275.2 300.8 256 300.8 243.2 313.6c-38.4 57.6-64 128-64 198.4 0 70.4 19.2 140.8 64 198.4 6.4 6.4 12.8 12.8 25.6 12.8 6.4 0 12.8 0 19.2-6.4 12.8-12.8 19.2-25.6 6.4-44.8C262.4 627.2 243.2 569.6 243.2 512c0-57.6 19.2-115.2 51.2-160C300.8 339.2 300.8 320 288 307.2zM838.4 512c0-70.4-19.2-140.8-64-198.4-12.8-12.8-25.6-19.2-44.8-6.4-12.8 12.8-19.2 25.6-6.4 44.8 32 44.8 51.2 102.4 51.2 160 0 57.6-19.2 115.2-51.2 160-12.8 12.8-6.4 32 6.4 44.8 6.4 6.4 12.8 6.4 19.2 6.4 6.4 0 19.2-6.4 25.6-12.8C819.2 652.8 838.4 582.4 838.4 512z"></path></svg>
					&nbsp;{$LL.Home.Layout.LAN()}
				</a>
				<a href="/home/history" class="btn flex-1 {$page.url.pathname == '/home/history' ? 'btn-active' : undefined}">
					<svg class="h-5 w-5" fill="white" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"><path d="M411.733333 885.333333H192c-6.4 0-10.666667-4.266667-10.666667-10.666666V149.333333c0-6.4 4.266667-10.666667 10.666667-10.666666h576c6.4 0 10.666667 4.266667 10.666667 10.666666v219.733334c0 17.066667 14.933333 32 32 32s32-14.933333 32-32V149.333333c0-40.533333-34.133333-74.666667-74.666667-74.666666H192C151.466667 74.666667 117.333333 108.8 117.333333 149.333333v725.333334c0 40.533333 34.133333 74.666667 74.666667 74.666666h219.733333c17.066667 0 32-14.933333 32-32s-14.933333-32-32-32z"></path><path d="M704 458.666667c-134.4 0-245.333333 110.933333-245.333333 245.333333S569.6 949.333333 704 949.333333 949.333333 838.4 949.333333 704 838.4 458.666667 704 458.666667z m0 426.666666c-100.266667 0-181.333333-81.066667-181.333333-181.333333s81.066667-181.333333 181.333333-181.333333 181.333333 81.066667 181.333333 181.333333-81.066667 181.333333-181.333333 181.333333z"></path><path d="M802.133333 716.8l-66.133333-29.866667V597.333333c0-17.066667-14.933333-32-32-32s-32 14.933333-32 32v110.933334c0 12.8 8.533333 23.466667 19.2 29.866666l85.333333 38.4c4.266667 2.133333 8.533333 2.133333 12.8 2.133334 12.8 0 23.466667-6.4 29.866667-19.2 6.4-17.066667 0-34.133333-17.066667-42.666667zM693.333333 298.666667c0-17.066667-14.933333-32-32-32H298.666667c-17.066667 0-32 14.933333-32 32s14.933333 32 32 32h362.666666c17.066667 0 32-14.933333 32-32zM298.666667 437.333333c-17.066667 0-32 14.933333-32 32s14.933333 32 32 32h106.666666c17.066667 0 32-14.933333 32-32s-14.933333-32-32-32h-106.666666z"></path></svg>
					&nbsp;{$LL.Home.Layout.History()}
				</a>
			</div>
		</div>

		<div class="custom-scroll flex-1 overflow-y-auto">
			<slot />
		</div>

		<div class="flex-none text-center">
			<hr />
			<a
				class="align-text-top text-xs text-blue-500 hover:text-blue-600"
				rel="noreferrer"
				target="_blank"
				href="https://github.com/MirrorX-Desktop/MirrorX"
			>
				MirrorX
			</a>
		</div>
	</div>
</div>

<DialogVisitRequest />
<DialogInputRemotePassword />
<DialogSelectLanguage />
<HomeNotificationCenter />

<style>
	.custom-scroll::-webkit-scrollbar {
		width: 14px;
	}

	.custom-scroll::-webkit-scrollbar-thumb {
		border: 4px solid rgba(0, 0, 0, 0);
		background-clip: padding-box;
		border-radius: 9999px;
		background-color: #aaaaaa;
	}

	.custom-scroll::-webkit-scrollbar-track {
		@apply my-4;
	}
</style>
