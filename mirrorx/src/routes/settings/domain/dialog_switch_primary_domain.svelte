<script lang="ts">
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke_config_domain_get, invoke_config_domain_update } from '$lib/components/command';
	import { onDestroy, onMount } from 'svelte';
	import { emitSettingsNotification } from '../settings_notification_center.svelte';
	import type { SwitchPrimaryDomainEvent } from './event';
	import LL from '$lib/i18n/i18n-svelte';
	import { current_domain } from '$lib/components/stores';

	let show: boolean = false;
	let unlisten_fn: UnlistenFn | null = null;
	let domain_id: number = 0;
	let domain_name: string = '';

	onMount(async () => {
		unlisten_fn = await listen<SwitchPrimaryDomainEvent>(
			'settings:domain:show_switch_primary_domain_dialog',
			(event) => {
				domain_id = event.payload.domain_id;
				domain_name = event.payload.domain_name;
				show = true;
			}
		);
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}
	});

	const yes = async () => {
		try {
			await invoke_config_domain_update(domain_id, { set_primary: true });
			let new_primary_domain = await invoke_config_domain_get();
			current_domain.set(new_primary_domain);
			await emit('home:switch_primary_domain');
			await emit('settings:domain:update_domains');
		} catch (error: any) {
			await emitSettingsNotification({
				level: 'error',
				message: error.toString() as string
			});
		} finally {
			no();
		}
	};

	const no = () => {
		show = false;
	};
</script>

<slot>
	<input type="checkbox" id="dialog_switch_primary_domain" class="modal-toggle" checked={show} />
	<div class="modal">
		<div class="modal-box w-96">
			<h3 class="text-lg font-bold">{$LL.Settings.Pages.Dialog.SetPrimaryDomain.Title()}</h3>
			<div class="py-4">
				{$LL.Settings.Pages.Dialog.SetPrimaryDomain.ContentPrefix()}
				<span class="font-bold">{domain_name}</span>
				{$LL.Settings.Pages.Dialog.SetPrimaryDomain.ContentSuffix()}
			</div>
			<div class="modal-action">
				<button class="btn" on:click={yes}>{$LL.DialogActions.Yes()}</button>
				<button class="btn" on:click={no}>{$LL.DialogActions.No()}</button>
			</div>
		</div>
	</div>
</slot>
