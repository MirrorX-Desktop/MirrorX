<script lang="ts">
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import {
		invoke_config_domain_get_by_name,
		invoke_config_domain_get,
		invoke_config_domain_update,
		invoke_signaling_connect,
		invoke_signaling_visit
	} from '$lib/components/command';
	import { onDestroy, onMount } from 'svelte';
	import LL from '$lib/i18n/i18n-svelte';
	import { current_domain } from '$lib/components/stores';
	import { emitNotification } from '$lib/components/notification';
	import { faEye, faEyeSlash } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import { formatDeviceID } from '$lib/components/utility';
	import { isMacOS } from '$lib/components/types';
	import { get } from 'svelte/store';
	import { faSpinner } from '@fortawesome/free-solid-svg-icons';

	let show: boolean = false;
	let unlisten_fn: UnlistenFn | null = null;
	let domain_id: number = 0;
	let domain_name: string = '';
	let input_password = '';
	let show_password = false;
	let remote_device_id: string = '';
	let is_connecting: boolean = false;

	onMount(async () => {
		unlisten_fn = await listen<{ domain_name: string; device_id: number }>('/dialog/history_connect', async (event) => {
			domain_name = event.payload.domain_name;
			domain_id = (await invoke_config_domain_get_by_name(domain_name)).id;
			remote_device_id = formatDeviceID(event.payload.device_id);
			show = true;
		});
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}
	});

	const yes = async () => {
		if (input_password.length == 0) {
			console.log('is not zero');
			return;
		}

		try {
			is_connecting = true;
			let primary_domain = get(current_domain);
			if (primary_domain?.name != domain_name) {
				await invoke_config_domain_update(domain_id, 'set_primary');
				await invoke_signaling_connect(true);
				let new_primary_domain = await invoke_config_domain_get();
				current_domain.set(new_primary_domain);
				await emit('update_domains');
			}
			await invoke_signaling_visit(remote_device_id, input_password);
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
			no();
		}
	};

	const no = () => {
		show = false;
		remote_device_id = '';
		is_connecting = false;
		input_password = '';
		show_password = false;
		domain_name = '';
		domain_id = 0;
	};
</script>

<slot>
	<input type="checkbox" id="dialog_history_connect" class="modal-toggle" checked={show} />
	<div data-tauri-drag-region class="modal {isMacOS ? '' : 'rounded-lg'}">
		<div class="modal-box">
			<div class="pb-4">
				<p class="py-1 text-center text-2xl font-bold">{domain_name}</p>
				<p class="py-1 text-center">({$LL.Dialogs.HistoryConnect.Tip()})</p>
				<p class="py-1 text-center text-xl font-bold">{remote_device_id}</p>
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

			<div class="modal-action flex">
				<button class="btn flex-1 {is_connecting ? 'btn-disabled' : ''}" on:click={yes}>
					{#if is_connecting}
						<Fa icon={faSpinner} spin />
					{:else}
						{$LL.DialogActions.Ok()}
					{/if}
				</button>
				<button class="btn flex-1" on:click={no}>{$LL.DialogActions.Cancel()}</button>
			</div>
		</div>
	</div>
</slot>
