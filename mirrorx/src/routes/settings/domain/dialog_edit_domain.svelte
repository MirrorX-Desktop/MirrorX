<script lang="ts">
	import { faCircleExclamation } from '@fortawesome/free-solid-svg-icons';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { emitSettingsNotification } from '../settings_notification_center.svelte';
	import type { EditDomainEvent } from './event';
	import LL from '$lib/i18n/i18n-svelte';
	import { invoke_config_domain_update } from '$lib/components/command';

	let show: boolean = false;
	let domain_id: number = 0;
	let domain_name: string = '';
	let domain_device_id: string = '';
	let domain_finger_print: string = '';
	let domain_remarks: string = '';
	let unlisten_fn: UnlistenFn | null = null;

	onMount(async () => {
		unlisten_fn = await listen<EditDomainEvent>('settings:domain:show_edit_domain_dialog', (event) => {
			domain_id = event.payload.domain_id;
			domain_name = event.payload.domain_name;
			domain_finger_print = event.payload.domain_finger_print;
			domain_remarks = event.payload.domain_remarks;

			let device_id_str = String(event.payload.domain_device_id).padStart(10, '0');
			domain_device_id = `${device_id_str.substring(0, 2)}
			-
			${device_id_str.substring(2, 6)}
			-
			${device_id_str.substring(6, 10)}`;

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
			await invoke_config_domain_update(domain_id, { remarks: domain_remarks });
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
		<div class="modal-box max-w-min overflow-hidden">
			<h3 class="text-lg font-bold">{$LL.Settings.Pages.Dialog.EditDomain.Title()}</h3>
			<div class="py-4">
				<div class="pb-2">
					<table class="table w-full">
						<tbody>
							<tr>
								<th class="w-1/3 text-right">{$LL.Settings.Pages.Dialog.EditDomain.Name()}</th>
								<td class="text-center">{domain_name}</td>
							</tr>
							<tr>
								<th class="text-right">{$LL.Settings.Pages.Dialog.EditDomain.DeviceId()}</th>
								<td class="text-center">{domain_device_id}</td>
							</tr>
							<tr>
								<th class="whitespace-normal text-right">
									<div
										class="tooltip tooltip-right flex items-center justify-end gap-1"
										data-tip={$LL.Settings.Pages.Dialog.EditDomain.FingerPrint.Tooltip()}
									>
										<span class="min-w-fit break-normal"
											>{$LL.Settings.Pages.Dialog.EditDomain.FingerPrint.Label()}</span
										>
										<Fa icon={faCircleExclamation} />
									</div>
								</th>
								<td class="text-center">{domain_finger_print}</td>
							</tr>
							<tr>
								<th class="text-right" style="z-index:0!important">{$LL.Settings.Pages.Dialog.EditDomain.Remarks()}</th>
								<td>
									<input type="text" maxlength="15" bind:value={domain_remarks} class="w-full rounded border p-2" />
								</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
			<div class="modal-action">
				<button class="btn" on:click={ok}>{$LL.DialogActions.Ok()}</button>
				<button class="btn" on:click={cancel}>{$LL.DialogActions.Cancel()}</button>
			</div>
		</div>
	</div>
</slot>
