<script lang="ts">
	import { invoke_get_lan_discover_nodes } from '$lib/components/command';
	import { current_lan_discover_nodes, type LanDiscoverNode } from '$lib/components/stores';
	import { onDestroy, onMount } from 'svelte';
	import type { Unsubscriber } from 'svelte/store';
	import { emitHomeNotification } from '../home_notification_center.svelte';
	import Panel from './panel.svelte';

	let timer: NodeJS.Timer | null = null;

	let nodes: Array<LanDiscoverNode> = [];
	let lan_discover_nodes_unsubscriber: Unsubscriber | null = null;

	onMount(async () => {
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
			nodes = await invoke_get_lan_discover_nodes();
			current_lan_discover_nodes.set(nodes);
		} catch (error: any) {
			await emitHomeNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};
</script>

<slot>
	<div class="h-full">
		<div class="flex-none">
			<table class="w-full">
				{#each nodes as node}
					<Panel
						host_name={node.host_name}
						addr={node.addr}
						os={node.os}
						os_version={node.os_version}
						tcp_port={node.tcp_port}
						udp_port={node.udp_port}
					/>
				{/each}
			</table>
		</div>
	</div>
</slot>
