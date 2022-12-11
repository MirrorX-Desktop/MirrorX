<script lang="ts">
	import { invoke_lan_nodes_list } from '$lib/components/command';
	import { current_lan_discover_nodes } from '$lib/components/stores';
	import type { LanDiscoverNode } from '$lib/components/types';
	import { onDestroy, onMount } from 'svelte';
	import type { Unsubscriber } from 'svelte/store';
	import { get } from 'svelte/store';
	import { emitHomeNotification } from '../notification_home.svelte';
	import Panel from './panel.svelte';

	let timer: NodeJS.Timer | null = null;

	let nodes: Array<LanDiscoverNode> = [];
	let lan_discover_nodes_unsubscriber: Unsubscriber | null = null;

	onMount(async () => {
		nodes = get(current_lan_discover_nodes);

		lan_discover_nodes_unsubscriber = current_lan_discover_nodes.subscribe((v) => {
			nodes = v;
		});

		timer = setInterval(get_lan_discover_nodes, 20 * 1000);
	});

	onDestroy(() => {
		if (timer != null) {
			clearInterval(timer);
		}

		if (lan_discover_nodes_unsubscriber != null) {
			lan_discover_nodes_unsubscriber();
		}
	});

	const get_lan_discover_nodes = async () => {
		try {
			nodes = await invoke_lan_nodes_list();
			current_lan_discover_nodes.set(nodes);
		} catch (error: any) {
			await emitHomeNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};
</script>

<slot>
	<table class="w-full">
		{#each nodes as node}
			<Panel
				host_name={node.host_name}
				addr={node.addr}
				os={node.os}
				os_version={node.os_version}
				is_orphan={nodes.length == 1}
			/>
		{/each}
	</table>
</slot>
