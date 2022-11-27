<script lang="ts">
	import { invoke_get_lan_discover_nodes } from '$lib/components/command';
	import { onDestroy, onMount } from 'svelte';
	import { emitHomeNotification } from '../home_notification_center.svelte';

	let timer: NodeJS.Timer | null = null;

	let nodes: Array<{
		host_name: string;
		addr: string;
		os: string;
		os_version: string;
		tcp_port: number;
		udp_port: number;
	}> = [];

	onMount(async () => {
		timer = setInterval(get_lan_discover_nodes, 20 * 1000);
	});

	onDestroy(() => {
		if (timer != null) {
			clearInterval(timer);
		}
	});

	const get_lan_discover_nodes = async () => {
		try {
			nodes = await invoke_get_lan_discover_nodes();
		} catch (error: any) {
			await emitHomeNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};
</script>

<slot
	><div class="align-center flex h-full flex-col place-items-center justify-center">
		<div class="flex-none">
			{#each nodes as node}
				<div class="border">{node.host_name}</div>
			{/each}
		</div>
	</div>
</slot>
