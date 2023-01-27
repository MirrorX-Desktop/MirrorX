<script lang="ts">
	import { faEye, faEyeSlash } from '@fortawesome/free-solid-svg-icons';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke_lan_connect, invoke_signaling_visit } from '$lib/components/command';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { emitNotification } from '$lib/components/notification';
	import LL from '$lib/i18n/i18n-svelte';
	import { isMacOS } from '$lib/components/types';

	let addr: string = '';
	let hostname: string = '';
	let show = false;
	let unlisten_fn: UnlistenFn | null;

	onMount(async () => {
		unlisten_fn = await listen<{
			addr: string;
			hostname: string;
		}>('/dialog/lan_connect', (event) => {
			addr = event.payload.addr;
			hostname = event.payload.hostname;
			show = true;
		});
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}
	});

	const ok = async (visitDesktop: boolean) => {
		try {
			show = false;
			await invoke_lan_connect(addr, visitDesktop);
		} catch (error: any) {
			console.log(error);
			await emitNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};

	const cancel = async () => {
		show = false;
	};
</script>

<slot>
	<input type="checkbox" id="dialog_lan_connect" class="modal-toggle" bind:checked={show} />
	<div data-tauri-drag-region class="modal {isMacOS ? '' : 'rounded-lg'}">
		<div class="modal-box">
			<div class="py-4">
				<p class="py-1 text-center text-xl font-bold">{hostname}</p>
				<p class="py-1 text-center text-lg">{addr}</p>
				<p class="pt-1 text-center text-lg">{$LL.Dialogs.LANConnect.Content()}</p>
			</div>
			<div class="flex flex-col gap-2">
				<div class="flex flex-1 flex-row gap-2">
					<button class="btn flex-1" on:click={() => ok(true)}>{$LL.Home.Desktop()}</button>
					<button class="btn flex-1" on:click={() => ok(false)}>{$LL.Home.Files()}</button>
				</div>
				<div>
					<button class="btn-outline btn-ghost btn w-full" on:click={cancel}>
						{$LL.DialogActions.Cancel()}
					</button>
				</div>
			</div>
		</div>
	</div>
</slot>
