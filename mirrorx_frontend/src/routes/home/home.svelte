<script lang="ts">
	import { faSpinner, faPlus, faSliders } from '@fortawesome/free-solid-svg-icons';
	import { emit } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import LL from '../../i18n/i18n-svelte';
	import type { GetCurrentDomainResponse, NotificationEvent } from '../event_types';
	import Connect from './connect.svelte';
	import History from './history.svelte';
	import Lan from './lan.svelte';
	import DialogInputRemotePassword from '../widgets/dialog_input_remote_password.svelte';
	import DialogVisitRequest from '../widgets/dialog_visit_request.svelte';
	import NotificationCenter from '../widgets/notification_center.svelte';

	var select_tab: string = 'connect';
	var current_domain: GetCurrentDomainResponse;

	onMount(async () => {
		init();
	});

	const switch_tab = (tab_name: string) => (select_tab = tab_name);

	const init = async () => {
		try {
			await invoke('init_config');
			await invoke('init_signaling', { force: false });
			current_domain = await invoke<GetCurrentDomainResponse>('get_current_domain');
		} catch (error: any) {
			let notification: NotificationEvent = {
				level: 'error',
				title: 'Error',
				message: error.toString()
			};
			emit('notification', notification);
		}
	};

	const open_settings_window = () => {
		const webview = new WebviewWindow('window_settings', {
			url: '/settings',
			resizable: false,
			maximized: false,
			maxWidth: 680,
			height: 580,
			center: true,
			title: 'Settings'
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
			<div class="text-2xl">{$LL.Domain()}</div>

			<div class="dropdown dropdown-end">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<label tabindex="0" class="btn btn-xs"><Fa icon={faSliders} /></label>

				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box w-52 p-2 shadow">
					<li><button on:click={open_settings_window}>Edit Domain</button></li>
				</ul>
			</div>
		</div>
	</div>

	<div class="mx-2 flex flex-1 flex-col">
		<div class="flex-none">
			<div class="my-2 text-center text-4xl">
				{#if current_domain == undefined}
					<Fa class="w-full text-center" icon={faSpinner} spin={true} size={'sm'} />
				{/if}
				{#if current_domain != undefined}
					{current_domain.name}
				{/if}
			</div>
			<div class="btn-group my-3 flex flex-row">
				<button
					class="btn flex-1 {select_tab == 'connect' ? 'btn-active' : undefined}"
					on:click={() => switch_tab('connect')}>{$LL.Connect()}</button
				>
				<button class="btn flex-1 {select_tab == 'lan' ? 'btn-active' : undefined}" on:click={() => switch_tab('lan')}
					>{$LL.LAN()}</button
				>
				<button
					class="btn flex-1 {select_tab == 'history' ? 'btn-active' : undefined}"
					on:click={() => switch_tab('history')}>{$LL.History()}</button
				>
			</div>
		</div>

		<div class="flex-1">
			{#if current_domain != undefined}
				{#if select_tab == 'connect'}
					<Connect bind:domain={current_domain} />
				{/if}
				{#if select_tab == 'lan'}
					<Lan />
				{/if}
				{#if select_tab == 'history'}
					<History />
				{/if}
			{:else}
				<Fa icon={faSpinner} spin />
			{/if}
		</div>

		<div class="flex-none text-center">
			<hr />
			<a
				class="align-text-top text-xs text-blue-500 hover:text-blue-600"
				href="https://github.com/MirrorX-Desktop/MirrorX">MirrorX</a
			>
		</div>
	</div>
</div>

<DialogVisitRequest />
<DialogInputRemotePassword />
<NotificationCenter />
