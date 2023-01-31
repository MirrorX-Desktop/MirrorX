<script lang="ts">
	import type { LanDiscoverNode } from '$lib/components/types';
	import { faXmark, faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import * as commands from '$lib/components/command';
	import Panel from './panel.svelte';
	import { emitNotification } from '$lib/components/notification';
	import LL from '$lib/i18n/i18n-svelte';
	import { debounce } from 'debounce';

	const PAGE_COUNT = 7;

	let timer: NodeJS.Timer | null = null;
	let use_search_result: boolean = false;
	let nodes: Array<LanDiscoverNode> = [];
	let search_nodes: Array<LanDiscoverNode> = [];
	let display_nodes: Array<LanDiscoverNode> = [];
	let display_page: number = 1;
	let display_total_pages: number = 1;
	let discoverable: boolean = true;

	$: has_prev_page = display_page != 1;
	$: has_next_page =
		display_page < Math.ceil((use_search_result ? search_nodes.length : nodes.length) / PAGE_COUNT);

	onMount(async () => {
		discoverable = await commands.invoke_lan_discoverable_get();

		await get_lan_discover_nodes();
		timer = setInterval(get_lan_discover_nodes, 10 * 1000);
	});

	onDestroy(() => {
		if (timer != null) {
			clearInterval(timer);
		}
	});

	const get_lan_discover_nodes = async () => {
		try {
			nodes = await commands.invoke_lan_nodes_list();
			if (!use_search_result) {
				refresh_pagination_limit();
				refresh_result();
			}
		} catch (error: any) {
			await emitNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};

	const search_lan_nodes = debounce(async (keyword: string) => {
		if (keyword.length == 0) {
			use_search_result = false;
			nodes = await commands.invoke_lan_nodes_list();
		} else {
			use_search_result = true;
			search_nodes = await commands.invoke_lan_nodes_search(keyword);
		}

		refresh_pagination_limit();
		refresh_result();
	}, 500);

	const prev_page = () => {
		if (has_prev_page) {
			display_page -= 1;
			refresh_result();
		}
	};

	const next_page = () => {
		if (has_next_page) {
			display_page += 1;
			refresh_result();
		}
	};

	const refresh_pagination_limit = () => {
		if (use_search_result) {
			display_total_pages = Math.ceil(search_nodes.length / PAGE_COUNT);
		} else {
			display_total_pages = Math.ceil(nodes.length / PAGE_COUNT);
		}

		if (display_total_pages < 1) {
			display_total_pages = 1;
		}

		if (display_page > display_total_pages) {
			display_page = display_total_pages;
		}
	};

	const refresh_result = () => {
		let start = PAGE_COUNT * (display_page - 1);
		if (use_search_result) {
			display_nodes = search_nodes.slice(start, start + PAGE_COUNT);
		} else {
			display_nodes = nodes.slice(start, start + PAGE_COUNT);
		}
	};

	const changeDiscoverable = debounce(async (checked: boolean) => {
		if (discoverable == checked) {
			return;
		}

		try {
			discoverable = checked;
			await commands.invoke_lan_discoverable_set(discoverable);
		} catch (err: any) {
			await emitNotification({
				level: 'error',
				title: 'Error',
				message: err.toString()
			});
		}
	}, 1000);
</script>

<slot>
	<div class="flex h-full w-full flex-col gap-2 py-2 px-2">
		<div class="flex flex-row gap-2">
			<input
				id="search_input"
				type="text"
				placeholder={$LL.LAN.HostnameOrIP()}
				class="input-bordered input input-sm flex-1 text-center focus:border-info focus:outline-none focus:ring focus:ring-info"
				on:input={(ev) => search_lan_nodes(ev.currentTarget.value)}
			/>
		</div>
		<div class="flex flex-row justify-center">
			<div>
				<div class="text-xs text-base-content text-opacity-50">
					{$LL.LAN.DiscoveredDevicesTip()}
				</div>
			</div>
		</div>
		<div id="panel" class="w-full flex-1 overflow-y-auto">
			<!-- at most 7 panel here -->
			<div class="flex flex-col ">
				{#each display_nodes as node}
					<Panel
						display_name={node.display_name}
						addrs={node.addrs}
						os={node.os}
						os_version={node.os_version}
					/>
				{/each}
			</div>
		</div>
		<div class="flex items-center justify-between">
			<div class="form-control">
				<label class="label flex cursor-pointer items-center gap-1">
					<input
						type="checkbox"
						checked={discoverable}
						on:change={(ev) => changeDiscoverable(ev.currentTarget.checked)}
						class="checkbox-primary checkbox checkbox-xs"
					/>
					<span class="label-text">{$LL.LAN.Discoverable()}</span>
				</label>
			</div>
			<div class="btn-group">
				<button class="btn-xs btn" on:click={prev_page}>«</button>
				<button class="btn-xs btn">{display_page}&nbsp;/&nbsp;{display_total_pages}</button>
				<button class="btn-xs btn" on:click={next_page}>»</button>
			</div>
		</div>
	</div>
</slot>

<style>
	#panel::-webkit-scrollbar {
		@apply w-1;
	}

	#panel::-webkit-scrollbar-thumb {
		@apply rounded-full bg-base-300;
	}

	#panel::-webkit-scrollbar-track {
		@apply bg-transparent;
	}
</style>
