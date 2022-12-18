<script lang="ts">
	import { faCircleExclamation } from '@fortawesome/free-solid-svg-icons';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import LL from '$lib/i18n/i18n-svelte';
	import { invoke_config_domain_update } from '$lib/components/command';
	import { formatDeviceID } from '$lib/components/utility';
	import { emitNotification } from '$lib/components/notification';
	import { isMacOS } from '$lib/components/types';

	let show: boolean = false;
	let domain_id: number = 0;
	let domain_name: string = '';
	let domain_device_id: string = '';
	let domain_finger_print: string = '';
	let domain_remarks: string = '';
	let unlisten_fn: UnlistenFn | null = null;

	onMount(async () => {
		unlisten_fn = await listen<{
			domain_id: number;
			domain_name: string;
			domain_device_id: number;
			domain_finger_print: string;
			domain_remarks: string;
		}>('/dialog/domain_edit', (event) => {
			domain_id = event.payload.domain_id;
			domain_name = event.payload.domain_name;
			domain_device_id = formatDeviceID(event.payload.domain_device_id);
			domain_finger_print = event.payload.domain_finger_print;
			domain_remarks = event.payload.domain_remarks;
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
			await emitNotification({
				level: 'error',
				title: 'Error',
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
	<div data-tauri-drag-region class="modal {isMacOS ? '' : 'rounded-lg'}">
		<div class="modal-box overflow-hidden">
			<h3 class="text-lg font-bold">{$LL.Dialogs.DomainEdit.Title()}</h3>
			<div class="py-4">
				<div class="pb-2">
					<div class="divider text-sm">{$LL.Dialogs.DomainEdit.Name()}</div>
					<div class="text-center text-lg">{domain_name}</div>

					<div class="divider text-sm">{$LL.Dialogs.DomainEdit.DeviceId()}</div>
					<div class="text-center text-lg">{domain_device_id}</div>

					<div class="divider text-sm">
						<div class="tooltip tooltip-top whitespace-normal" data-tip={$LL.Dialogs.DomainEdit.FingerPrint.Tooltip()}>
							<div class="flex flex-row items-center justify-center gap-1 whitespace-nowrap">
								<div>{$LL.Dialogs.DomainEdit.FingerPrint.Label()}</div>
								<Fa icon={faCircleExclamation} />
							</div>
						</div>
					</div>
					<div class="text-center text-lg">{domain_finger_print}</div>

					<div class="divider text-sm">{$LL.Dialogs.DomainEdit.Remarks()}</div>
					<div>
						<input
							type="text"
							maxlength="15"
							bind:value={domain_remarks}
							class="input input-bordered ring-info focus:border-info w-full flex-1 p-2 text-center focus:outline-none focus:ring"
						/>
					</div>
				</div>
			</div>
			<div class="modal-action mt-0">
				<button class="btn" on:click={ok}>{$LL.DialogActions.Ok()}</button>
				<button class="btn" on:click={cancel}>{$LL.DialogActions.Cancel()}</button>
			</div>
		</div>
	</div>
</slot>
