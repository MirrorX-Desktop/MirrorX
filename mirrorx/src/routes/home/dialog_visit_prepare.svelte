<script lang="ts">
	import { faEye, faEyeSlash } from '@fortawesome/free-solid-svg-icons';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke_signaling_visit } from '$lib/components/command';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { emitHomeNotification } from './notification_home.svelte';
	import LL from '$lib/i18n/i18n-svelte';

	let remote_device_id: string = '';
	let show = false;
	let input_password = '';
	let show_password = false;
	let unlisten_fn: UnlistenFn | null;

	onMount(async () => {
		unlisten_fn = await listen<{
			remote_device_id: string;
		}>('/dialog/visit/prepare/open', (event) => {
			remote_device_id = event.payload.remote_device_id;
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
			await invoke_signaling_visit(remote_device_id, input_password);
		} catch (error: any) {
			console.log(error);
			let err: string = error.toString();
			if (err.includes('Internal')) {
				err = 'Remote Device Internal Error';
			} else if (err.includes('InvalidArgs')) {
				err = 'Invalid Request Args Used at Key Exchange';
			} else if (err.includes('InvalidPassword')) {
				err = 'Incorrect Password';
			}

			await emitHomeNotification({ level: 'error', title: 'Error', message: err.toString() });
		} finally {
			remote_device_id = '';
			input_password = '';
			show_password = false;
			await emit('desktop_is_connecting', false);
		}
	};

	const cancel = async () => {
		remote_device_id = '';
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
			<h3 class="text-lg font-bold">{$LL.Home.Pages.Connect.Dialog.InputRemotePassword.Title()}</h3>
			<div class="py-4">
				<p class="py-1 text-lg">{$LL.Home.Pages.Connect.Dialog.InputRemotePassword.ContentPrefix()}</p>
				<p class="py-1 text-center text-xl font-bold">{remote_device_id}</p>
				<p class="py-1 text-lg">{$LL.Home.Pages.Connect.Dialog.InputRemotePassword.ContentSuffix()}</p>
			</div>
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
				<button class="btn" on:click={ok}>{$LL.DialogActions.Ok()}</button>
				<button class="btn" on:click={cancel}>{$LL.DialogActions.Cancel()}</button>
			</div>
		</div>
	</div>
</slot>
