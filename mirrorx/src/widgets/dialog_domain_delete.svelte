<script lang="ts">
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke_config_domain_delete } from '$lib/components/command';
	import { onDestroy, onMount } from 'svelte';
	import LL from '$lib/i18n/i18n-svelte';
	import { emitNotification } from '$lib/components/notification';
	import { isMacOS } from '$lib/components/types';

	let show: boolean = false;
	let unlisten_fn: UnlistenFn | null = null;
	let domain_id: number = 0;
	let domain_name: string = '';

	onMount(async () => {
		unlisten_fn = await listen<{ domain_id: number; domain_name: string }>('/dialog/domain_delete', (event) => {
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
			await invoke_config_domain_delete(domain_id);
			await emit('update_domains');
		} catch (error: any) {
			await emitNotification({
				level: 'error',
				title: 'Error',
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
	<div data-tauri-drag-region class="modal {isMacOS ? '' : 'rounded-lg'}">
		<div class="modal-box">
			<h3 class="text-lg font-bold">{$LL.Dialogs.DomainDelete.Title()}</h3>
			<div class="py-4">
				{$LL.Dialogs.DomainDelete.ContentPrefix()}
				<span class="font-bold">{domain_name}</span>
				{$LL.Dialogs.DomainDelete.ContentSuffix()}
			</div>
			<div class="modal-action">
				<button class="btn" on:click={yes}>{$LL.DialogActions.Yes()}</button>
				<button class="btn" on:click={no}>{$LL.DialogActions.No()}</button>
			</div>
		</div>
	</div>
</slot>
