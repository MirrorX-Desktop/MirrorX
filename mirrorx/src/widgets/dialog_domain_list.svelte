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

	const SINGLE_PAGE_LIMIT: number = 6;

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

		update_domains_unlisten_fn = await listen('settings:domain:update_domains', (event) => {
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

	const show_delete_confirm_dialog = async (id: number, name: string) => {
		await emit('/dialog/domain_delete', {
			domain_id: id,
			domain_name: name
		});
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

	const show_switch_domain_dialog = async (id: number, name: string) => {
		await emit('/dialog/domain_switch', {
			domain_id: id,
			domain_name: name
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

				<hr />

				{#if resp != null}
					<div id="domain-table" class="h-60 max-h-60 w-full overflow-y-auto overflow-x-hidden">
						<table class="table-compact table w-full">
							<tbody>
								{#each resp.domains as domain, i}
									<tr>
										<th style="z-index: 0 !important;">{(page - 1) * SINGLE_PAGE_LIMIT + i + 1}</th>
										<td>
											<p class="text-xl">{domain.name}</p>
											<p class="text-xs">{domain.remarks}</p>
										</td>

										<td class="text-right">
											<div class="btn-group ">
												{#if domain.name != primary_domain?.name}
													<button
														class="btn btn-xs tooltip tooltip-bottom"
														data-tip={$LL.Dialogs.DomainList.Tooltips.SetPrimary()}
														on:click={() => show_switch_domain_dialog(domain.id, domain.name)}
													>
														<Fa icon={faThumbTack} />
													</button>
												{/if}

												<button
													class="btn btn-xs tooltip tooltip-bottom"
													data-tip={$LL.Dialogs.DomainList.Tooltips.Edit()}
													on:click={() =>
														show_edit_domain_dialog(
															domain.id,
															domain.name,
															domain.device_id,
															domain.finger_print,
															domain.remarks
														)}
												>
													<Fa icon={faPenToSquare} />
												</button>

												{#if domain.name != primary_domain?.name && domain.name != 'MirrorX.cloud'}
													<button
														class="btn btn-xs tooltip tooltip-bottom"
														data-tip={$LL.Dialogs.DomainList.Tooltips.Delete()}
														on:click={() => show_delete_confirm_dialog(domain.id, domain.name)}
													>
														<Fa icon={faTrashCan} />
													</button>
												{/if}
											</div>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				{:else}
					<Fa icon={faSpinner} spin />
				{/if}
				<hr />
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
