<!-- <script lang="ts">
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import { emitNotification } from '$lib/components/notification';
	import LL from '$lib/i18n/i18n-svelte';

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
			// await invoke_signaling_reply_visit_request({
			// 	allow,
			// 	activeDeviceId: active_device_id,
			// 	passiveDeviceId: passive_device_id
			// });
		} catch (error: any) {
			await emitNotification({ level: 'error', title: 'Error', message: error.toString() });
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
			<h3 class="text-lg font-bold">{$LL.Home.Pages.Connect.Dialog.VisitRequest.Title()}</h3>
			<div class="py-4">
				<p class="py-1 text-lg">{$LL.Home.Pages.Connect.Dialog.VisitRequest.ContentPrefix()}</p>
				<p class="py-1 text-center text-xl font-bold">{passive_device_id}</p>
				<p class="py-1 text-lg">
					{$LL.Home.Pages.Connect.Dialog.VisitRequest.ContentSuffix()}
					<span class="font-bold">
						{resource_type == 'desktop'
							? $LL.Home.Pages.Connect.Dialog.VisitRequest.ResourceType.Desktop()
							: $LL.Home.Pages.Connect.Dialog.VisitRequest.ResourceType.Files()}
					</span>
				</p>
			</div>
			<div class="modal-action">
				<button class="btn" on:click={() => decide(true)}>
					{$LL.DialogActions.Allow()} (
					<span class="countdown">
						<span style="--value:{countdown};" />
					</span>
					)
				</button>
				<button class="btn" on:click={() => decide(false)}>{$LL.DialogActions.Reject()}</button>
			</div>
		</div>
	</div>
</slot> -->
