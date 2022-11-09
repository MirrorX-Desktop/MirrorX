<script lang="ts">
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke_switch_primary_domain } from '../../../components/command';
	import { onDestroy, onMount } from 'svelte';
	import { emitSettingsNotification } from '../settings_notification_center.svelte';
	import type { SwitchPrimaryDomainEvent } from './event';

	let show: boolean = false;
	let unlisten_fn: UnlistenFn | null = null;
	let domain_id: number = 0;
	let domain_name: string = '';

	onMount(async () => {
		unlisten_fn = await listen<string>('settings:domain:show_switch_primary_domain_dialog', (event) => {
			let ev: SwitchPrimaryDomainEvent = JSON.parse(event.payload);
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

	const yes = async () => {
		try {
			await invoke_switch_primary_domain({ id: domain_id });
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
			<h3 class="text-lg font-bold">Switch Primary Domain</h3>
			<div class="py-4">Do you want to set <span class="font-bold">{domain_name}</span> as primary domain?</div>
			<div class="modal-action">
				<button class="btn" on:click={yes}>Yes</button>
				<button class="btn" on:click={no}>No</button>
			</div>
		</div>
	</div>
</slot>
