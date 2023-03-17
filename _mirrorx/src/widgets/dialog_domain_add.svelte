<script lang="ts">
	import { faSpinner, faXmarkCircle } from '@fortawesome/free-solid-svg-icons';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke_config_domain_create } from '$lib/components/command';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import LL from '$lib/i18n/i18n-svelte';
	import { isMacOS } from '$lib/components/types';

	let show: boolean = false;
	let input_domain_address: string = '';
	let input_domain_remarks: string = '';
	let validating: boolean = false;
	let error_text: string = '';
	let cancel_fn: ((reason?: any) => void) | null = null;
	let unlisten_fn: UnlistenFn | null = null;

	onMount(async () => {
		unlisten_fn = await listen('/dialog/domain_add', (_) => {
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
			validating = true;
			error_text = '';

			// todo: here just cancel js promise on the frontend side,
			// but how to cancel backend task also?
			let cancel_promise = new Promise((resolve, reject) => {
				cancel_fn = reject;
			});

			let invoke_promise = invoke_config_domain_create(input_domain_address, input_domain_remarks);

			await Promise.race([invoke_promise, cancel_promise]);

			await emit('update_domains');

			show = false;
			input_domain_address = '';
			input_domain_remarks = '';
		} catch (error: any) {
			if (error != 'cancelled') {
				error_text = error.toString();
			}
		} finally {
			cancel_fn = null;
			validating = false;
		}
	};

	const cancel = () => {
		if (cancel_fn) {
			cancel_fn('cancelled');
			cancel_fn = null;
		}
		show = false;
		validating = false;
		input_domain_address = '';
		input_domain_remarks = '';
	};
</script>

<slot>
	<input type="checkbox" id="dialog_add_domain" class="modal-toggle" bind:checked={show} />
	<div data-tauri-drag-region class="modal {isMacOS ? '' : 'rounded-lg'}">
		<div class="modal-box">
			<h3 class="text-lg font-bold">{$LL.Dialogs.DomainAdd.Title()}</h3>
			{#if error_text != ''}
				<div class="alert alert-error shadow-lg">
					<div>
						<Fa icon={faXmarkCircle} />
						<span>{error_text}</span>
					</div>
				</div>
			{/if}
			<div class="py-4">
				<div class="pb-2">
					<input
						type="text"
						bind:value={input_domain_address}
						placeholder={$LL.Dialogs.DomainAdd.AddressInputPlaceHolder()}
						class="input input-bordered ring-info focus:border-info w-full flex-1 p-2 focus:outline-none focus:ring"
					/>
				</div>
				<div class="pt-2">
					<input
						type="text"
						bind:value={input_domain_remarks}
						placeholder={$LL.Dialogs.DomainAdd.RemarksInputPlaceHolder()}
						class="input input-bordered ring-info focus:border-info w-full flex-1 p-2 focus:outline-none focus:ring"
					/>
				</div>
			</div>
			<div class="modal-action">
				<button class="btn {validating ? 'btn-disabled' : ''}" on:click={ok}>
					{#if validating}
						<Fa icon={faSpinner} spin />
					{:else}
						<span>{$LL.DialogActions.Ok()}</span>
					{/if}
				</button>
				<button class="btn" on:click={cancel}>{$LL.DialogActions.Cancel()}</button>
			</div>
		</div>
	</div>
</slot>
