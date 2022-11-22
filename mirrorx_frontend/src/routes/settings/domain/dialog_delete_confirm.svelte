<script lang="ts">
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import { invoke_delete_domain } from '../../../components/command';
	import { onDestroy, onMount } from 'svelte';
	import { emitSettingsNotification } from '../settings_notification_center.svelte';
	import type { DeleteConfirmEvent } from './event';
	import LL from '$lib/i18n/i18n-svelte';

	let show: boolean = false;
	let unlisten_fn: UnlistenFn | null = null;
	let domain_id: number = 0;
	let domain_name: string = '';

	onMount(async () => {
		unlisten_fn = await listen<DeleteConfirmEvent>('settings:domain:show_delete_confirm_dialog', (event) => {
			domain_id = event.payload.domain_id;
			domain_name = event.payload.domain_name;
			show = true;
		});
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}
	});

	const yes = async () => {
		try {
			await invoke_delete_domain({ id: domain_id });
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
	<input type="checkbox" id="dialog_delete_confirm" class="modal-toggle" checked={show} />
	<div class="modal">
		<div class="modal-box w-96">
			<h3 class="text-lg font-bold">{$LL.Settings.Pages.Dialog.DeleteDomain.Title()}</h3>
			<div class="py-4">
				{$LL.Settings.Pages.Dialog.DeleteDomain.ContentPrefix()}
				<span class="font-bold">{domain_name}</span>
				{$LL.Settings.Pages.Dialog.DeleteDomain.ContentSuffix()}
			</div>
			<div class="modal-action">
				<button class="btn" on:click={yes}>{$LL.DialogActions.Yes()}</button>
				<button class="btn" on:click={no}>{$LL.DialogActions.No()}</button>
			</div>
		</div>
	</div>
</slot>
