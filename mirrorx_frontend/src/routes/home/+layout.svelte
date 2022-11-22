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
	import { current_domain, type CurrentDomain } from '../../components/stores';
	import {
		invoke_get_current_domain,
		invoke_get_language,
		invoke_init_config,
		invoke_init_signaling
	} from '../../components/command';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import DialogSelectLanguage from './dialog_select_language.svelte';
	import type { UpdateLanguageEvent } from '$lib/components/rust_event';
	import type { Locales } from '$lib/i18n/i18n-types';
	import { clipboard } from '@tauri-apps/api';

	let domain: CurrentDomain | null = null;
	let domain_unsubscribe: Unsubscriber | null = null;
	let switch_primary_unlisten_fn: UnlistenFn | null = null;
	let update_language_unlisten_fn: UnlistenFn | null = null;

	onMount(async () => {
		domain_unsubscribe = current_domain.subscribe((value) => {
			console.log('layout update domain');
			domain = value;
		});

		switch_primary_unlisten_fn = await listen('home:switch_primary_domain', switch_primary_domain);
		update_language_unlisten_fn = await listen<UpdateLanguageEvent>('update_language', (event) =>
			setLocale(event.payload.language as Locales)
		);
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

	(async function () {
		try {
			await invoke_init_config();
			console.log('finish init config');

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
	<div data-tauri-drag-region class="mx-2 flex flex-col">
		<div data-tauri-drag-region class=" z-10 mt-2 mb-2 flex flex-row items-center justify-between">
			<button class="btn btn-xs invisible"><Fa icon={faSliders} /></button>
			<div class="text-2xl">{$LL.Home.Layout.Domain()}</div>

			<div class="dropdown dropdown-end">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<label tabindex="0" class="btn btn-xs"><Fa icon={faSliders} /></label>

				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box w-52 p-2 shadow">
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

	<div class="mx-2 flex flex-1 flex-col">
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
					{$LL.Home.Layout.Connect()}
				</a>
				<a href="/home/lan" class="btn flex-1 {$page.url.pathname == '/home/lan' ? 'btn-active' : undefined}">
					{$LL.Home.Layout.LAN()}
				</a>
				<a href="/home/history" class="btn flex-1 {$page.url.pathname == '/home/history' ? 'btn-active' : undefined}">
					{$LL.Home.Layout.History()}
				</a>
			</div>
		</div>

		<div class="flex-1">
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
