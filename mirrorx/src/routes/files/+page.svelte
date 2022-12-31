<script lang="ts">
	import { page } from '$app/stores';
	import { invoke_file_manager_visit } from '$lib/components/command';
	import type { Directory } from '$lib/components/types';
	import { onMount } from 'svelte/types/runtime/internal/lifecycle';

	let remote_device_id: string = $page.url.searchParams.get('device_id')!;
	let currentDirectory: Directory | null = null;

	onMount(async () => {
		await request_dir();
	});

	const request_dir = async () => {
		let path: string | null = null;
		if (currentDirectory) {
			if (currentDirectory.path != '/' && currentDirectory.path != '\\') {
				path = currentDirectory.path;
			}
		}

		currentDirectory = await invoke_file_manager_visit(remote_device_id, path);
	};
</script>

<div>
	{#if currentDirectory}
		{#each currentDirectory.sub_dirs as dir}
			<div>{dir.path}&nbsp;{dir.modified_time}</div>
		{/each}
		{#each currentDirectory.files as file}
			<div>{file.path}&nbsp;{file.modified_time}&nbsp;{file.size}</div>
		{/each}
	{/if}
</div>
