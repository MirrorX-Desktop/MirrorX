<script lang="ts">
	import {
		faChevronLeft,
		faChevronRight,
		faCircleExclamation,
		faPenToSquare,
		faPlus,
		faSpinner,
		faThumbTack,
		faTrash,
		faTrashCan,
		faXmark
	} from '@fortawesome/free-solid-svg-icons';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { invoke_config_domain_get, invoke_config_domain_list } from '$lib/components/command';
	import LL from '$lib/i18n/i18n-svelte';
	import { isMacOS, type Domain } from '$lib/components/types';
	import { formatDeviceID } from '$lib/components/utility';
	import { emitNotification } from '$lib/components/notification';

	const SINGLE_PAGE_LIMIT: number = 5;

	let show: boolean = false;
	let page = 1;
	let primary_domain: Domain | null = null;
	let resp: {
		total: number;
		domains: Array<Domain>;
	} | null = null;
	let show_dialog_domain_list_unlisten_fn: UnlistenFn | null = null;
	let update_domains_unlisten_fn: UnlistenFn | null = null;

	$: has_prev_page = page != 1;
	$: has_next_page = page < Math.ceil((resp?.total ?? 0) / SINGLE_PAGE_LIMIT);

	onMount(async () => {
		show_dialog_domain_list_unlisten_fn = await listen('/dialog/domain_list', (event) => {
			get_domains();
			show = true;
		});

		update_domains_unlisten_fn = await listen('update_domains', (event) => {
			get_domains();
		});
	});

	onDestroy(() => {
		if (show_dialog_domain_list_unlisten_fn) {
			show_dialog_domain_list_unlisten_fn();
		}

		if (update_domains_unlisten_fn) {
			update_domains_unlisten_fn();
		}
	});

	const get_domains = async () => {
		try {
			primary_domain = await invoke_config_domain_get();
			resp = await invoke_config_domain_list(page, SINGLE_PAGE_LIMIT);
		} catch (error: any) {
			await emitNotification({
				level: 'error',
				title: 'Error',
				message: error.toString() as string
			});
		}
	};

	const next_page = async () => {
		if (has_next_page) {
			page += 1;
			await get_domains();
		}
	};

	const prev_page = async () => {
		if (has_prev_page) {
			page -= 1;
			await get_domains();
		}
	};

	const show_add_domain_dialog = async () => {
		await emit('/dialog/domain_add');
	};

	const show_edit_domain_dialog = async (
		id: number,
		name: string,
		device_id: number,
		finger_print: string,
		remarks: string
	) => {
		await emit('/dialog/domain_edit', {
			domain_id: id,
			domain_name: name,
			domain_device_id: device_id,
			domain_finger_print: finger_print,
			domain_remarks: remarks
		});
	};
</script>

<!-- svelte-ignore a11y-label-has-associated-control -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<slot>
	<input type="checkbox" id="dialog_domain_list" class="modal-toggle" bind:checked={show} />
	<div data-tauri-drag-region class="modal {isMacOS ? '' : 'rounded-lg'}">
		<div class="modal-box">
			<div class="mx-2 flex flex-col">
				<label class="btn btn-xs btn-outline btn-circle absolute right-2 top-2" on:click={() => (show = false)}>
					<Fa icon={faXmark} />
				</label>

				<div class="w-60 overflow-hidden text-ellipsis whitespace-nowrap py-3">
					{$LL.Dialogs.DomainList.Current() + primary_domain?.name ?? ''}
				</div>

				<hr class="mb-2" />

				{#if resp != null}
					<div id="domain-table" class="h-72 max-h-72 w-full overflow-y-auto overflow-x-hidden">
						<div class="w-full">
							{#each resp.domains as domain, i}
								<button
									class="hover:bg-primary hover:text-primary-content flex w-full cursor-pointer flex-row items-center rounded-lg p-2 transition-all hover:rounded-lg"
									on:click={() =>
										show_edit_domain_dialog(
											domain.id,
											domain.name,
											domain.device_id,
											domain.finger_print,
											domain.remarks
										)}
								>
									<div class="pr-2">
										<div class="w-8 text-center">{(page - 1) * SINGLE_PAGE_LIMIT + i + 1}</div>
									</div>
									<div class="w-full flex-1 text-left">
										<p class="w-48 overflow-hidden text-ellipsis text-xl">{domain.name}</p>
										<p class="text-xs">{formatDeviceID(domain.device_id)}</p>
										{#if domain.remarks.length > 0}
											<p class="text-xs">{domain.remarks}</p>
										{/if}
									</div>
								</button>
							{/each}
						</div>
					</div>
				{:else}
					<Fa icon={faSpinner} spin />
				{/if}
				<hr class="mt-2" />
				<div class="flex flex-row items-center justify-between py-2">
					<div class="tooltip tooltip-top" data-tip={$LL.Dialogs.DomainList.Tooltips.Add()}>
						<button class="btn btn-xs" on:click={show_add_domain_dialog}><Fa icon={faPlus} /></button>
					</div>

					<div class="btn-group">
						<button class="btn btn-xs {!has_prev_page ? 'btn-disabled' : ''}" on:click={prev_page}>
							<Fa icon={faChevronLeft} />
						</button>
						<button class="btn btn-xs {!has_next_page ? 'btn-disabled' : ''}" on:click={next_page}>
							<Fa icon={faChevronRight} />
						</button>
					</div>
				</div>
			</div>
		</div>
	</div>
</slot>

<style>
	#domain-table::-webkit-scrollbar {
		@apply absolute right-0 w-2 rounded;
	}

	#domain-table::-webkit-scrollbar-thumb {
		@apply absolute right-0 rounded bg-gray-600;
	}

	#domain-table::-webkit-scrollbar-track {
		@apply absolute right-0 w-2 rounded bg-gray-50;
	}
</style>
