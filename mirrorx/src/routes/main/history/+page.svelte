<script lang="ts">
	import { invoke_config_history_get } from '$lib/components/command';
	import type { HistoryRecord } from '$lib/components/types';
	import { onMount } from 'svelte';
	import moment from 'moment';
	import Fa from 'svelte-fa';
	import { faCircle } from '@fortawesome/free-solid-svg-icons';
	import { formatDeviceID } from '$lib/components/utility';
	import { emit } from '@tauri-apps/api/event';
	import LL from '$lib/i18n/i18n-svelte';
	import { emitNotification } from '$lib/components/notification';
	import { faSpinner } from '@fortawesome/free-solid-svg-icons';

	let timeRange: [number, number] | null = null;
	let timeRecords: Array<[string, Array<[string, Array<HistoryRecord>]>]> = [];
	let is_querying: boolean = false;

	onMount(async () => {
		await query_history();
	});

	const query_all = async () => {
		timeRange = null;
		await query_history();
	};

	const query_1_day = async () => {
		timeRange = [moment.utc().subtract(1, 'd').unix(), moment.utc().unix()];
		await query_history();
	};

	const query_7_days = async () => {
		timeRange = [moment.utc().subtract(7, 'd').unix(), moment.utc().unix()];
		await query_history();
	};

	const query_30_days = async () => {
		timeRange = [moment.utc().subtract(30, 'd').unix(), moment.utc().unix()];
		await query_history();
	};

	const query_history = async () => {
		try {
			is_querying = true;

			let records = await invoke_config_history_get(timeRange);
			let lastInsertDate = '';
			timeRecords = [];

			for (const record of records) {
				let t = moment.unix(record.timestamp).utc();
				let date = t.local().format('YYYY-MM-DD');

				if (lastInsertDate != date) {
					lastInsertDate = date;
					timeRecords.push([date, [[record.domain, [record]]]]);
				} else {
					let tuple = timeRecords[timeRecords.length - 1][1];
					let index = tuple.findIndex((v) => v[0] == record.domain);
					if (index != -1) {
						tuple[index][1].push(record);
					} else {
						tuple.push([record.domain, [record]]);
					}
				}
			}
		} catch (err: any) {
			await emitNotification({
				level: 'error',
				title: 'Error',
				message: err.toString() as string
			});
		} finally {
			is_querying = false;
		}
	};

	const connect = async (record: HistoryRecord) => {
		await emit('/dialog/history_connect', { domain_name: record.domain, device_id: record.device_id });
	};
</script>

<slot>
	<div class="flex h-full w-full flex-col overflow-hidden p-2">
		{#if is_querying}
			<div class="h-full w-full items-center justify-center">
				<Fa icon={faSpinner} spin />
			</div>
		{:else}
			<div class="btn-group mb-2 flex w-full max-w-full">
				<button class="btn btn-sm flex-1" on:click={query_all}>{$LL.History.All()}</button>
				<button class="btn btn-sm flex-1" on:click={query_1_day}>{$LL.History.Day1()}</button>
				<button class="btn btn-sm flex-1" on:click={query_7_days}>{$LL.History.Days7()}</button>
				<button class="btn btn-sm flex-1" on:click={query_30_days}>{$LL.History.Days30()}</button>
			</div>
			<div id="panel" class="overflow-y-auto pr-2">
				<ol class="border-primary ml-2" style="border-left-width: 1.5px;">
					{#each timeRecords as record}
						<li class="relative">
							<div class="flex-start absolute flex items-center" style="left:-8px;">
								<Fa icon={faCircle} size="sm" class="text-primary" />
								<h4 class="text-base-content pl-2 text-xl font-semibold">{record[0]}</h4>
							</div>
							<div class="ml-4 flex flex-col pb-2 pt-8">
								{#each record[1] as group}
									<div class="divider my-2">{group[0]}</div>
									{#each group[1] as item}
										<button class="btn btn-sm btn-outline my-1" on:click={() => connect(item)}>
											{formatDeviceID(item.device_id)}
										</button>
									{/each}
								{/each}
							</div>
						</li>
					{/each}
				</ol>
			</div>
		{/if}
	</div>
</slot>

<style>
	#panel::-webkit-scrollbar {
		@apply w-1;
	}

	#panel::-webkit-scrollbar-thumb {
		@apply bg-base-300 rounded-full;
	}

	#panel::-webkit-scrollbar-track {
		@apply bg-transparent;
	}
</style>
