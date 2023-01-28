<script lang="ts">
	import { invoke_file_manager_query_transferred_bytes_count } from '$lib/components/command';
	import type { FileTransferItem } from '$lib/components/types';
	import {
		formatFileSize,
		formatSecondsDuration,
		formatTransferSpeed
	} from '$lib/components/utility';
	import {
		faArrowRightArrowLeft,
		faDownload,
		faRightLeft,
		faUpload
	} from '@fortawesome/free-solid-svg-icons';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import moment from 'moment';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import LL from '$lib/i18n/i18n-svelte';

	let currentTab: 'transferring' | 'transferSucceed' | 'transferFailed' = 'transferring';
	let transferring_items: Array<FileTransferItem> = [];
	let transfer_succeed_items: Array<FileTransferItem> = [];
	let transfer_failed_items: Array<FileTransferItem> = [];

	let add_file_transfer_item_unlisten_fn: UnlistenFn | null = null;
	let update_progress_timer: NodeJS.Timer | null = null;

	onMount(async () => {
		add_file_transfer_item_unlisten_fn = await listen<FileTransferItem>(
			'add_file_transfer_item',
			async (event) => {
				transferring_items.push(event.payload);
				transferring_items = transferring_items;
			}
		);

		update_progress_timer = setInterval(async () => {
			await updateFinishedItems();
			await updateTransferringItems();
		}, 1000);
	});

	onDestroy(() => {
		if (add_file_transfer_item_unlisten_fn) {
			add_file_transfer_item_unlisten_fn();
		}

		if (update_progress_timer) {
			clearInterval(update_progress_timer);
		}
	});

	const switchTab = (tabName: 'transferring' | 'transferSucceed' | 'transferFailed') => {
		if (currentTab != tabName) {
			currentTab = tabName;
		}
	};

	const updateFinishedItems = async () => {
		let finished_items = transferring_items.filter(
			(item) => item.total_size == item.transferred_size
		);

		finished_items.forEach((item) => (item.succeed_at = moment().unix()));
		transfer_succeed_items.push(...finished_items);
		transfer_succeed_items = transfer_succeed_items;

		let still_transferring_items = transferring_items.filter(
			(item) => item.total_size != item.transferred_size
		);

		transferring_items = still_transferring_items;
	};

	const updateTransferringItems = async () => {
		for (let i = 0; i < transferring_items.length; i++) {
			let item = transferring_items[i];
			let bytes = await invoke_file_manager_query_transferred_bytes_count(item.id);

			item.last_transferred_delta_size = bytes - item.transferred_size;
			item.transferred_size = bytes;
		}

		transferring_items = transferring_items;
	};
</script>

<div class="flex h-52 w-full flex-col">
	<div class="flex-0 tabs z-20 -mb-px w-full ">
		<button
			class="tab tab-lifted {currentTab == 'transferring'
				? 'tab-active'
				: '[--tab-border-color:transparent]'}"
			on:click={() => switchTab('transferring')}
		>
			{$LL.FileTransfer.Transfer.Transferring()}
		</button>
		<button
			class="tab tab-lifted {currentTab == 'transferSucceed' ? 'tab-active' : undefined}"
			on:click={() => switchTab('transferSucceed')}
		>
			{$LL.FileTransfer.Transfer.TransferSucceed()}
		</button>
		<button
			class="tab tab-lifted {currentTab == 'transferFailed' ? 'tab-active' : undefined}"
			on:click={() => switchTab('transferFailed')}
		>
			{$LL.FileTransfer.Transfer.TransferFailed()}
		</button>
		<div class="tab tab-lifted mr-6 flex-1 cursor-default" />
	</div>

	<div
		class="z-10 flex flex-1 flex-col overflow-hidden rounded-lg border border-base-300
        {currentTab == 'transferring' ? 'rounded-tl-none' : undefined}"
	>
		{#if currentTab == 'transferring'}
			<table class="table-compact table w-full table-fixed">
				<thead>
					<tr>
						<th class="w-8 rounded-none" />
						<th class="w-1/5 text-center text-xs font-normal"
							>{$LL.FileTransfer.Transfer.TableLocalPath()}</th
						>
						<th class="w-6 text-center text-xs font-normal"><Fa icon={faArrowRightArrowLeft} /></th>
						<th class="w-1/5 text-center text-xs font-normal"
							>{$LL.FileTransfer.Transfer.TableRemotePath()}</th
						>
						<th class="w-1/5 text-right text-xs font-normal"
							>{$LL.FileTransfer.Transfer.TableTransferredAndTotalSize()}</th
						>
						<th class="w-1/6 text-right text-xs font-normal"
							>{$LL.FileTransfer.Transfer.TableProgress()}</th
						>
						<th class="rounded-none text-xs font-normal"
							>{$LL.FileTransfer.Transfer.TableStatus()}</th
						>
					</tr>
				</thead>
			</table>

			<div class="transfer-view z-0 h-full w-full flex-1 overflow-auto">
				<table class="table-compact table w-full table-fixed">
					<tbody>
						{#each transferring_items as item, index}
							<tr>
								<th class="w-8 rounded-none text-center">{index + 1}</th>
								<td class="w-1/5 overflow-hidden overflow-ellipsis text-center text-xs font-normal">
									{item.local_path}
								</td>
								<td class="w-6 text-center text-xs font-normal">
									{#if item.is_upload}
										<Fa icon={faUpload} />
									{:else}
										<Fa icon={faDownload} />
									{/if}
								</td>
								<td class="w-1/5 overflow-hidden overflow-ellipsis text-center text-xs font-normal">
									{item.remote_path}
								</td>
								<td class="w-1/5 text-right text-xs font-normal">
									{`${formatFileSize(item.transferred_size)} / ${formatFileSize(item.total_size)}`}
								</td>
								<td class="w-1/6 text-xs font-normal">
									<progress
										class="progress progress-success w-full"
										value={item.transferred_size}
										max={item.total_size}
									/>
									<div class="flex w-full flex-row justify-between">
										<div>
											{#if item.total_size == item.transferred_size}
												&nbsp;
											{:else}
												{formatTransferSpeed(item.last_transferred_delta_size)}
											{/if}
										</div>
										<div>{((item.transferred_size / item.total_size) * 100).toFixed(2)}%</div>
									</div>
								</td>
								<td class="rounded-none text-xs font-normal">
									{$LL.FileTransfer.Transfer.Transferring()}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{:else if currentTab == 'transferSucceed'}
			<table class="table-compact table w-full table-fixed">
				<thead>
					<tr>
						<th class="w-8 rounded-none" />
						<th class="w-1/5 text-center text-xs font-normal"
							>{$LL.FileTransfer.Transfer.TableLocalPath()}</th
						>
						<th class="w-6 text-center text-xs font-normal"><Fa icon={faArrowRightArrowLeft} /></th>
						<th class="w-1/5 text-center text-xs font-normal"
							>{$LL.FileTransfer.Transfer.TableRemotePath()}</th
						>
						<th class="w-1/6 text-center text-xs font-normal"
							>{$LL.FileTransfer.Transfer.TableTotalSize()}</th
						>
						<th class="w-1/5 text-xs font-normal">{$LL.FileTransfer.Transfer.TableFinishAt()}</th>
						<th class="rounded-none text-xs font-normal"
							>{$LL.FileTransfer.Transfer.TableTimeCost()}</th
						>
					</tr>
				</thead>
			</table>

			<div class="transfer-view z-0 h-full w-full flex-1 overflow-auto">
				<table class="table-compact table w-full table-fixed">
					<tbody>
						{#each transfer_succeed_items as item, index}
							<tr>
								<th class="w-8 rounded-none text-center">{index + 1}</th>
								<td class="w-1/5 overflow-hidden overflow-ellipsis text-center text-xs font-normal">
									{item.local_path}
								</td>
								<td class="w-6 text-center text-xs font-normal">
									{#if item.is_upload}
										<Fa icon={faUpload} />
									{:else}
										<Fa icon={faDownload} />
									{/if}
								</td>
								<td class="w-1/5 overflow-hidden overflow-ellipsis text-center text-xs font-normal">
									{item.remote_path}
								</td>
								<td class="w-1/6 text-center text-xs font-normal">
									{`${formatFileSize(item.total_size)}`}
								</td>
								<td class="w-1/5 text-xs font-normal">
									{moment.unix(item.succeed_at).format('YYYY-MM-DD hh:mm:ss')}
								</td>
								<td class="rounded-none text-xs font-normal">
									{formatSecondsDuration(item.succeed_at - item.launch_at)}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</div>
</div>

<style>
	.transfer-view::-webkit-scrollbar {
		@apply w-1;
	}

	.transfer-view::-webkit-scrollbar-thumb {
		@apply rounded-full bg-base-300;
	}

	.transfer-view::-webkit-scrollbar-track {
		@apply my-2 bg-transparent;
	}
</style>
