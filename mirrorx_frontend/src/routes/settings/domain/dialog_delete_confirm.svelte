<script lang="ts">
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import { invoke_delete_domain } from '../../../components/command';
	import { onDestroy, onMount } from 'svelte';
	import { emitSettingsNotification } from '../settings_notification_center.svelte';
	import type { DeleteConfirmEvent } from './event';

	let show: boolean = false;
	let unlisten_fn: UnlistenFn | null = null;
	let domain_id: number = 0;
	let domain_name: string = '';

	onMount(async () => {
		unlisten_fn = await listen<string>('settings:domain:show_delete_confirm_dialog', (event) => {
			let ev: DeleteConfirmEvent = JSON.parse(event.payload);
			domain_id = ev.domain_id;
			domain_name = ev.domain_name;
			show = true;
		});
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}
	});

	const ok = async () => {
		try {
			await invoke_delete_domain({ id: domain_id });
			await emit('settings:domain:update_domains');
		} catch (error: any) {
			await emitSettingsNotification({
				level: 'error',
				message: error.toString() as string
			});
		} finally {
			cancel();
		}
	};

	const cancel = () => {
		show = false;
	};
</script>

<slot>
	<input type="checkbox" id="dialog_delete_confirm" class="modal-toggle" checked={show} />
	<div class="modal">
		<div class="modal-box w-96">
			<h3 class="text-lg font-bold">Delete Domain</h3>
			<div class="py-4">Do you really want to delete domain <span class="font-bold">{domain_name}</span>?</div>
			<div class="modal-action">
				<button class="btn" on:click={ok}>Ok</button>
				<button class="btn" on:click={cancel}>Cancel</button>
			</div>
		</div>
	</div>
</slot>
