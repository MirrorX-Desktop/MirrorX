<script lang="ts">
	import { faEye, faEyeSlash } from '@fortawesome/free-solid-svg-icons';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import type { NotificationEvent, PopupDialogInputRemotePasswordEvent } from '../event_types';

	var event: PopupDialogInputRemotePasswordEvent | null;
	var show = false;
	var input_password: string = '';
	var show_password = false;
	var popup_dialog_input_remote_password_unlisten_fn: UnlistenFn | null;

	onMount(async () => {
		popup_dialog_input_remote_password_unlisten_fn = await listen<PopupDialogInputRemotePasswordEvent>(
			'popup_dialog_input_remote_password',
			(ev) => {
				event = ev.payload;
				show = true;
			}
		);
	});

	onDestroy(() => {
		if (popup_dialog_input_remote_password_unlisten_fn) {
			popup_dialog_input_remote_password_unlisten_fn();
		}
	});

	const decide = async (allow: boolean) => {
		show = false;

		try {
			await invoke('signaling_key_exchange', {
				localDeviceId: event?.active_device_id,
				remoteDeviceId: event?.passive_device_id,
				password: input_password
			});
		} catch (error: any) {
			let notification: NotificationEvent = {
				level: 'error',
				title: 'Error',
				message: error.toString()
			};
			emit('notification', notification);
		}

		event = null;
		input_password = '';
		show_password = false;
		console.log('emit desktop is connecting');
		emit('desktop_is_connecting', false);
	};
</script>

<slot>
	<input type="checkbox" id="dialog_input_remote_password" class="modal-toggle" bind:checked={show} />
	<div class="modal">
		<div class="modal-box">
			<h3 class="text-lg font-bold">Input Remote Password</h3>
			<p class="py-4">
				Remote Device '<span class="font-bold">{event?.passive_device_id}</span>' pass your visit request. Please input
				remote device password
			</p>

			<div class="input-group flex flex-row">
				<input
					type={show_password ? 'text' : 'password'}
					class="w-full rounded border text-center focus:border-blue-300 focus:outline-none focus:ring"
					maxlength="20"
					value={input_password}
					on:input={(event) => (input_password = event.currentTarget.value)}
				/>

				<button
					class="btn btn-square flex-none"
					on:click={() => (show_password = !show_password)}
					on:mouseleave={() => (show_password = false)}
				>
					<Fa icon={show_password ? faEye : faEyeSlash} />
				</button>
			</div>

			<div class="modal-action">
				<button class="btn" on:click={() => decide(true)}>Ok</button>
				<button class="btn" on:click={() => decide(false)}>Cancel</button>
			</div>
		</div>
	</div>
</slot>
