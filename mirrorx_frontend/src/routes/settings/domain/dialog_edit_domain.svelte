<script lang="ts">
	import { faCircleExclamation, faSpinner } from '@fortawesome/free-solid-svg-icons';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke_add_domain, invoke_set_domain_remarks } from '../../../components/command';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { emitSettingsNotification } from '../settings_notification_center.svelte';
	import type { EditDomainEvent } from './event';

	let show: boolean = false;
	let domain_id: number = 0;
	let domain_name: string = '';
	let domain_device_id: string = '';
	let domain_finger_print: string = '';
	let domain_remarks: string = '';
	let unlisten_fn: UnlistenFn | null = null;

	onMount(async () => {
		unlisten_fn = await listen<string>('settings:domain:show_edit_domain_dialog', (event) => {
			let ev: EditDomainEvent = JSON.parse(event.payload);
			domain_id = ev.domain_id;
			domain_name = ev.domain_name;
			domain_device_id = ev.domain_device_id;
			domain_finger_print = ev.domain_finger_print;
			domain_remarks = ev.domain_remarks;
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
			await invoke_set_domain_remarks({ id: domain_id, remarks: domain_remarks });
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
	<input type="checkbox" id="dialog_visit_request" class="modal-toggle" bind:checked={show} />
	<div class="modal">
		<div class="modal-box overflow-visible">
			<h3 class="text-lg font-bold">Edit Domain</h3>
			<div class="py-4">
				<div class="pb-2">
					<table class="table w-full">
						<tbody>
							<tr>
								<th class="w-1/2 text-right">Name</th>
								<td class="text-center">{domain_name}</td>
							</tr>
							<tr>
								<th class="text-right">Device Id</th>
								<td class="text-center">{domain_device_id}</td>
							</tr>
							<tr>
								<th class="whitespace-normal">
									<div class="flex items-center gap-1">
										<span>FingerPrint</span>
										<div
											class="tooltip tooltip-bottom"
											data-tip="Fingerprint is a random string and will not track your device"
										>
											<Fa icon={faCircleExclamation} />
										</div>
									</div>
								</th>
								<td>{domain_finger_print}</td>
							</tr>
							<tr>
								<th class="text-right" style="z-index:0!important">Remarks</th>
								<td
									><input
										type="text"
										bind:value={domain_remarks}
										placeholder="Remarks"
										class="w-full rounded border p-2"
									/></td
								>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
			<div class="modal-action">
				<button class="btn" on:click={ok}>Ok</button>
				<button class="btn" on:click={cancel}>Cancel</button>
			</div>
		</div>
	</div>
</slot>
