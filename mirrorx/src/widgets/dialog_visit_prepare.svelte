<script lang="ts">
	import { faEye, faEyeSlash } from '@fortawesome/free-solid-svg-icons';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke_signaling_visit } from '$lib/components/command';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { emitNotification } from '$lib/components/notification';
	import LL from '$lib/i18n/i18n-svelte';
	import { isMacOS } from '$lib/components/types';

	let remote_device_id: string = '';
	let show = false;
	let input_password = '';
	let show_password = false;
	let visit_desktop: boolean = true;
	let unlisten_fn: UnlistenFn | null;

	onMount(async () => {
		unlisten_fn = await listen<{
			remote_device_id: string;
			visit_desktop: boolean;
		}>('/dialog/visit_prepare', (event) => {
			remote_device_id = event.payload.remote_device_id;
			visit_desktop = event.payload.visit_desktop;
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
			await invoke_signaling_visit(remote_device_id, input_password, visit_desktop);
		} catch (error: any) {
			let err: string = error.toString();
			if (err.includes('Internal')) {
				err = 'Remote Device Internal Error';
			} else if (err.includes('InvalidArgs')) {
				err = 'Invalid Request Args Used at Key Exchange';
			} else if (err.includes('InvalidPassword')) {
				err = 'Incorrect Password';
			}

			await emitNotification({ level: 'error', title: 'Error', message: err.toString() });
		} finally {
			remote_device_id = '';
			input_password = '';
			show_password = false;
			await emit('desktop_is_connecting', false);
		}
	};

	const cancel = async () => {
		show = false;
		remote_device_id = '';
		input_password = '';
		show_password = false;
		await emit('desktop_is_connecting', false);
	};
</script>

<slot>
	<input type="checkbox" id="dialog_visit_prepare" class="modal-toggle" bind:checked={show} />
	<div data-tauri-drag-region class="modal {isMacOS ? '' : 'rounded-lg'}">
		<div class="modal-box">
			<div class="pb-4">
				<p class="py-1 text-center text-3xl font-bold">{remote_device_id}</p>
				<p class="py-1 text-center">{$LL.Dialogs.VisitPrepare.Content()}</p>
			</div>
			<div class="input-group flex flex-row">
				<input
					type={show_password ? 'text' : 'password'}
					class="input input-bordered focus:border-info focus:ring-info w-full text-center focus:outline-none focus:ring"
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

			<div class="modal-action flex flex-row">
				<button class="btn flex-1" on:click={ok}>{$LL.DialogActions.Ok()}</button>
				<button class="btn flex-1" on:click={cancel}>{$LL.DialogActions.Cancel()}</button>
			</div>
		</div>
	</div>
</slot>
