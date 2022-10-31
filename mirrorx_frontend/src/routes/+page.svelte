<script lang="ts">
	import { faSpinner } from '@fortawesome/free-solid-svg-icons';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import '../app.css';
	import LL from '../i18n/i18n-svelte';
	import Connect from './home/connect.svelte';
	import History from './home/history.svelte';
	import Lan from './home/lan.svelte';
	import DialogInputRemotePassword from './widgets/dialog_input_remote_password.svelte';
	import DialogVisitRequest from './widgets/dialog_visit_request.svelte';

	var select_tab: string = 'connect';
	var primary_domain: string;

	onMount(() => {
		init();
	});

	const switch_tab = (tab_name: string) => (select_tab = tab_name);

	const init = async () => {
		try {
			await invoke('init_config');
			let domain: string = await invoke('get_config_primary_domain');
			await invoke('init_signaling_client', { domain });

			primary_domain = domain;
		} catch (error) {
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

<DialogVisitRequest />
<DialogInputRemotePassword />
