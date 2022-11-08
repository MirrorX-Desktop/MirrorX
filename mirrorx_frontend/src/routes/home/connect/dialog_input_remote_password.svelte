<script lang="ts">
	import { faEye, faEyeSlash } from '@fortawesome/free-solid-svg-icons';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke_signaling_key_exchange } from '../../../components/command';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { emitHomeNotification } from '../home_notification_center.svelte';

	var active_device_id: string = '';
	var passive_device_id: string = '';
	var show = false;
	var input_password: string = '';
	var show_password = false;
	var unlisten_fn: UnlistenFn | null;

	onMount(async () => {
		unlisten_fn = await listen<{
			active_device_id: string;
			passive_device_id: string;
		}>('popup_dialog_input_remote_password', (event) => {
			active_device_id = event.payload.active_device_id;
			passive_device_id = event.payload.passive_device_id;
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
			show = false;
			await invoke_signaling_key_exchange({
				localDeviceId: active_device_id,
				remoteDeviceId: passive_device_id,
				password: input_password
			});
		} catch (error: any) {
			await emitHomeNotification({ level: 'error', title: 'Error', message: error.toString() });
		} finally {
			active_device_id = '';
			passive_device_id = '';
			input_password = '';
			show_password = false;
			console.log('emit desktop is connecting');
			await emit('desktop_is_connecting', false);
		}
	};

	const cancel = async () => {
		active_device_id = '';
		passive_device_id = '';
		input_password = '';
		show_password = false;
		console.log('emit desktop is connecting');
		await emit('desktop_is_connecting', false);
	};
</script>

<slot>
	<input type="checkbox" id="dialog_input_remote_password" class="modal-toggle" bind:checked={show} />
	<div class="modal">
		<div class="modal-box">
			<h3 class="text-lg font-bold">Input Remote Password</h3>
			<p class="py-4">
				Remote Device '<span class="font-bold">{passive_device_id}</span>' pass your visit request. Please input remote
				device password
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
				<button class="btn" on:click={ok}>Ok</button>
				<button class="btn" on:click={cancel}>Cancel</button>
			</div>
		</div>
	</div>
</slot>
