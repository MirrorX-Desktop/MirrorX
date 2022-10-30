<script lang="ts">
	import '../app.css';
	import Connect from './home/connect.svelte';
	import History from './home/history.svelte';
	import Lan from './home/lan.svelte';
	import LL from '../i18n/i18n-svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { faSpinner } from '@fortawesome/free-solid-svg-icons';
	import { listen } from '@tauri-apps/api/event';
	import type { UnlistenFn } from '@tauri-apps/api/event';

	var select_tab: String = 'connect';
	var primary_domain: String;
	var unlisten: UnlistenFn;

	onMount(() => {
		init();
	});

	onDestroy(() => {
		if (unlisten) {
			unlisten();
		}
	});

	const switch_tab = (tab_name: String) => (select_tab = tab_name);

	const init = async () => {
		try {
			await invoke('init_config');
			let domain: String = await invoke('get_config_primary_domain');
			await invoke('init_signaling_client', { domain });
			await listen_publish_message();

			primary_domain = domain;
		} catch (error) {
			// todo: pop dialog
		}
	};

	const listen_publish_message = async () => {
		try {
			unlisten = await listen('publish_message', (event) => {
				console.log(event);
			});
		} catch (err) {
			// todo: pop dialog
		}
	};
</script>

<div class="flex h-full flex-col">
	<div class="flex-none">
		<div class="mt-1 mb-2 text-center text-2xl">{$LL.Domain()}</div>
		<div class="my-2 text-center text-4xl">
			{#if primary_domain == undefined}
				<Fa class="w-full text-center" icon={faSpinner} spin={true} size={'sm'} />
			{/if}
			{#if primary_domain != undefined}
				{primary_domain}
			{/if}
		</div>
		<div class="btn-group my-3 mx-2 flex flex-row">
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
		{#if select_tab == 'connect'}
			<Connect domain={primary_domain} />
		{/if}
		{#if select_tab == 'lan'}
			<Lan />
		{/if}
		{#if select_tab == 'history'}
			<History />
		{/if}
	</div>

	<div class="mx-2 flex-none text-center">
		<hr />
		<a
			class="align-text-top text-xs text-blue-500 hover:text-blue-600"
			href="https://github.com/MirrorX-Desktop/MirrorX">MirrorX</a
		>
	</div>
</div>
