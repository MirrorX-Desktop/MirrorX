<script lang="ts">
	import type { LanDiscoverNode } from '$lib/components/types';
	import { faXmark, faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import * as commands from '$lib/components/command';
	import Panel from './panel.svelte';
	import { emitNotification } from '$lib/components/notification';

	let timer: NodeJS.Timer | null = null;
	let nodes: Array<LanDiscoverNode> = [];

	onMount(async () => {
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
		} catch (error: any) {
			await emitNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};
</script>

<slot>
	<div class="flex h-full w-full flex-col py-2 px-2">
		<!-- {#if nodes.length > 0} -->
		<div class="flex flex-row gap-2">
			<input
				id="search_input"
				type="text"
				placeholder="Host name Or IP"
				class="input input-bordered input-sm flex-1 text-center focus:border-blue-300 focus:outline-none focus:ring"
			/>
			<div class="form-control">
				<div class="input-group input-group-sm">
					<button class="btn btn-square btn-sm"><Fa icon={faXmark} /></button>
					<button class="btn btn-square btn-sm">
						<Fa icon={faMagnifyingGlass} />
					</button>
				</div>
			</div>
		</div>
		<div class="w-full flex-1 overflow-hidden pt-2">
			<!-- at most 7 panel here -->
			<div class="flex flex-col ">
				{#each nodes as node}
					<Panel host_name={node.host_name} addr={node.addr} os={node.os} os_version={node.os_version} />
				{/each}
			</div>
		</div>
		<div class="flex items-center justify-between">
			<div class="form-control">
				<label class="label flex cursor-pointer items-center gap-1">
					<input type="checkbox" checked={true} class="checkbox checkbox-primary checkbox-xs" />
					<span class="label-text">Discoverable</span>
				</label>
			</div>
			<div class="btn-group">
				<button class="btn btn-xs">«</button>
				<button class="btn btn-xs">Page 22</button>
				<button class="btn btn-xs">»</button>
			</div>
		</div>
		<!-- {:else}
			<div class="flex h-full w-full items-center justify-center">
				<label>ddd</label>
			</div>
		{/if} -->
	</div>
</slot>

<style>
	/* .custom-scrollbar::-webkit-scrollbar {
		width: 10px;
	} */

	/* .custom-scrollbar::-webkit-scrollbar-thumb {
		border: 1px solid rgba(0, 0, 0, 0);
		border-radius: 9999px;
		background-color: #aaaaaa;
	} */

	/* .custom-scrollbar::-webkit-scrollbar-track {
		margin-top: 10px;
		margin-left: 10px;
		margin-bottom: 10px;
		margin-right: 10px;
	} */
</style>
