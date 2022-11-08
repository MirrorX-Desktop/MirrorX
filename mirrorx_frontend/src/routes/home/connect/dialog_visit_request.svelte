<script lang="ts">
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke_signaling_reply_visit_request } from '../../../components/command';
	import { onDestroy, onMount } from 'svelte';
	import { emitHomeNotification } from '../home_notification_center.svelte';

	let active_device_id: string = '';
	let passive_device_id: string = '';
	let resource_type: string = '';
	var countdown = 30;
	var show = false;
	var unlisten_fn: UnlistenFn | null;
	var countdownIntervalId: NodeJS.Timer | null;

	onMount(async () => {
		unlisten_fn = await listen<{
			active_device_id: string;
			passive_device_id: string;
			resource_type: string;
		}>('popup_dialog_visit_request', (ev) => {
			active_device_id = ev.payload.active_device_id;
			passive_device_id = ev.payload.passive_device_id;
			resource_type = ev.payload.resource_type;
			countdown = 30;
			show = true;
			countdownIntervalId = setInterval(() => {
				countdown--;
				if (countdown == 0) {
					clearCountdown();
					decide(false);
				}
			}, 1000);
		});
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}

		clearCountdown();
	});

	const decide = async (allow: boolean) => {
		try {
			show = false;
			await invoke_signaling_reply_visit_request({
				allow,
				activeDeviceId: active_device_id,
				passiveDeviceId: passive_device_id
			});
		} catch (error: any) {
			await emitHomeNotification({ level: 'error', title: 'Error', message: error.toString() });
		} finally {
			clearCountdown();
		}
	};

	const clearCountdown = () => {
		if (countdownIntervalId) {
			clearInterval(countdownIntervalId);
		}
	};
</script>

<slot>
	<input type="checkbox" id="dialog_visit_request" class="modal-toggle" bind:checked={show} />
	<div class="modal">
		<div class="modal-box">
			<h3 class="text-lg font-bold">Visit Request</h3>
			<p class="py-4">
				Remote Device '<span class="font-bold">{active_device_id}</span>' want to visit your
				<span class="font-bold">{resource_type}</span>
			</p>
			<div class="modal-action">
				<button class="btn" on:click={() => decide(true)}>
					Allow (
					<span class="countdown">
						<span style="--value:{countdown};" />
					</span>
					)
				</button>
				<button class="btn" on:click={() => decide(false)}>Reject</button>
			</div>
		</div>
	</div>
</slot>
