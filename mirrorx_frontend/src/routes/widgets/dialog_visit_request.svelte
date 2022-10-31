<script lang="ts">
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onDestroy, onMount } from 'svelte';
	import type { NotificationEvent, PopupDialogVisitRequestEvent } from '../event_types';

	var event: PopupDialogVisitRequestEvent | null;
	var countdown = 30;
	var show = false;
	var popup_dialog_visit_request_unlisten_fn: UnlistenFn | null;
	var countdownIntervalId: NodeJS.Timer | null;

	onMount(async () => {
		popup_dialog_visit_request_unlisten_fn = await listen<PopupDialogVisitRequestEvent>(
			'popup_dialog_visit_request',
			(ev) => {
				event = ev.payload;
				countdown = 30;
				show = true;
				countdownIntervalId = setInterval(() => {
					countdown--;
					if (countdown == 0) {
						clearCountdown();
						decide(false);
					}
				}, 1000);
			}
		);
	});

	onDestroy(() => {
		if (popup_dialog_visit_request_unlisten_fn) {
			popup_dialog_visit_request_unlisten_fn();
		}

		clearCountdown();
	});

	const decide = async (allow: boolean) => {
		show = false;
		try {
			await invoke('signaling_reply_visit_request', {
				allow,
				activeDeviceId: event?.active_device_id,
				passiveDeviceId: event?.passive_device_id
			});
		} catch (error: any) {
			let notification: NotificationEvent = {
				level: 'error',
				title: 'Error',
				message: error.toString()
			};
			emit('notification', notification);
		}

		clearCountdown();
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
				Remote Device '<span class="font-bold">{event?.active_device_id ?? ''}</span>' want to visit your
				<span class="font-bold">{event?.resource_type ?? 'Unknown'}</span>
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
