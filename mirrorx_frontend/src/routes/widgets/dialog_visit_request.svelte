<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import type { VisitRequest } from '../event_types';

	var visit_request: VisitRequest | null;
	var countdown = 30;
	var show = false;

	onMount(() => {
		listen<VisitRequest>('pop_dialog_visit_request', (event) => {
			visit_request = event.payload;
			countdown = 30;
			show = true;
			let intervalId = setInterval(() => {
				countdown--;
				if (countdown == 0) {
					clearInterval(intervalId);
					decide(false);
				}
			}, 1000);
		});
	});

	const decide = async (allow: boolean) => {
		show = false;
		try {
			await invoke('signaling_reply_visit_request', {
				allow,
				activeDeviceId: visit_request?.active_device_id,
				passiveDeviceId: visit_request?.passive_device_id
			});
		} catch (error) {
			console.log('decide error: ' + error);
			// todo: pop dialog
		}
	};
</script>

<slot>
	<input type="checkbox" id="dialog_visit_request" class="modal-toggle" bind:checked={show} />
	<div class="modal">
		<div class="modal-box">
			<h3 class="text-lg font-bold">Visit Request</h3>
			<p class="py-4">
				Remote Device '<span class="font-bold">{visit_request?.active_device_id ?? ''}</span>' want to visit your
				<span class="font-bold">{visit_request?.resource_type ?? 'Unknown'}</span>
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
