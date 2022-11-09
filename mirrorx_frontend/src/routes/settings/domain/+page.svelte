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
	import DialogAddDomain from './dialog_add_domain.svelte';
	import DialogDeleteConfirm from './dialog_delete_confirm.svelte';
	import type { DeleteConfirmEvent, EditDomainEvent, SwitchPrimaryDomainEvent } from './event';
	import { emitSettingsNotification } from '../settings_notification_center.svelte';
	import { invoke_get_domains } from '../../../components/command';
	import DialogSwitchPrimaryDomain from './dialog_switch_primary_domain.svelte';
	import { identity } from 'svelte/internal';
	import DialogEditDomain from './dialog_edit_domain.svelte';

	const SINGLE_PAGE_LIMIT: number = 6;

	let page = 1;
	let resp: {
		total: number;
		current_domain_name: string;
		domains: Array<{
			id: number;
			name: string;
			addr: string;
			device_id: string;
			finger_print: string;
			remarks: string;
		}>;
	} | null = null;
	let unlisten_fn: UnlistenFn | null = null;

	$: has_prev_page = page != 1;
	$: has_next_page = page < Math.ceil((resp?.total ?? 0) / SINGLE_PAGE_LIMIT);

	onMount(async () => {
		get_domains();
		unlisten_fn = await listen('settings:domain:update_domains', get_domains);
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}
	});

	const get_domains = async () => {
		try {
			resp = await invoke_get_domains({ page, limit: SINGLE_PAGE_LIMIT });
		} catch (error: any) {
			await emitSettingsNotification({
				level: 'error',
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
		await emit('settings:domain:show_add_domain_dialog');
	};

	const show_delete_confirm_dialog = async (id: number, name: string) => {
		let payload: DeleteConfirmEvent = {
			domain_id: id,
			domain_name: name
		};

		await emit('settings:domain:show_delete_confirm_dialog', payload);
	};

	const show_edit_domain_dialog = async (
		id: number,
		name: string,
		device_id: string,
		finger_print: string,
		remarks: string
	) => {
		let payload: EditDomainEvent = {
			domain_id: id,
			domain_name: name,
			domain_device_id: device_id,
			domain_finger_print: finger_print,
			domain_remarks: remarks
		};

		await emit('settings:domain:show_edit_domain_dialog', payload);
	};

	const show_switch_primary_domain_dialog = async (id: number, name: string) => {
		let payload: SwitchPrimaryDomainEvent = {
			domain_id: id,
			domain_name: name
		};

		await emit('settings:domain:show_switch_primary_domain_dialog', payload);
	};
</script>

<slot>
	<div class="mx-2 flex h-full flex-col">
		<div class="flex flex-row items-center justify-between py-3">
			<div>
				<span class="text-2xl">Current:</span>
				<span class="text-2xl">{resp?.current_domain_name ?? ''}</span>
			</div>
			<button class="btn btn-xs" on:click={show_add_domain_dialog}><Fa icon={faPlus} /></button>
		</div>
		<hr />

		{#if resp != null}
			<div id="domain-table" class="h-80 w-full flex-1 overflow-y-auto overflow-x-hidden">
				<table class="table w-full">
					<tbody>
						{#each resp.domains as domain, i}
							<tr>
								<th style="z-index: 0 !important;">{(page - 1) * SINGLE_PAGE_LIMIT + i + 1}</th>
								<td>
									<p class="text-xl">{domain.name}</p>
									<p class="text-xs">{domain.remarks}</p>
								</td>
								<td class="w-full text-center">
									<p>{domain.device_id}</p>
								</td>
								<td class="text-right">
									<div class="btn-group ">
										{#if domain.name != resp.current_domain_name}
											<button
												class="btn btn-xs"
												on:click={() => show_switch_primary_domain_dialog(domain.id, domain.name)}
											>
												<Fa icon={faThumbTack} />
											</button>
										{/if}

										<button
											class="btn btn-xs"
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

										{#if domain.name != resp.current_domain_name && domain.name != 'MirrorX.cloud'}
											<button class="btn btn-xs" on:click={() => show_delete_confirm_dialog(domain.id, domain.name)}>
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
		<div class="py-2 text-center">
			<div class="btn-group">
				<button class="btn btn-sm {!has_prev_page ? 'btn-disabled' : ''}" on:click={prev_page}>
					<Fa icon={faChevronLeft} />
				</button>
				<button class="btn btn-sm {!has_next_page ? 'btn-disabled' : ''}" on:click={next_page}>
					<Fa icon={faChevronRight} />
				</button>
			</div>
		</div>
	</div>

	<DialogAddDomain />
	<DialogDeleteConfirm />
	<DialogEditDomain />
	<DialogSwitchPrimaryDomain />
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
