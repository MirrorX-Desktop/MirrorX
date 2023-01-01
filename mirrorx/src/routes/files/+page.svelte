<script lang="ts">
	import { page } from '$app/stores';
	import { invoke_file_manager_visit } from '$lib/components/command';
	import type { Directory } from '$lib/components/types';
	import { faSpinner } from '@fortawesome/free-solid-svg-icons';
	import { onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import DirView from './dir_view.svelte';

	let remote_device_id: string = $page.url.searchParams.get('device_id')!;
	let currentDirectory: Directory | null = null;

	onMount(async () => {
		await request_dir(null);
	});

	const request_dir = async (path: string | null) => {
		currentDirectory = await invoke_file_manager_visit(remote_device_id, path);
	};
</script>

<div class="flex h-full w-full flex-row">
	{#if currentDirectory}
		<div class="h-full flex-1">
			<!-- <DirView directory={currentDirectory} clickItem={(path) => request_dir(path)} /> -->
		</div>
		<div class="h-full flex-1">
			<DirView directory={currentDirectory} clickItem={(path) => request_dir(path)} />
		</div>
	{:else}
		<div><Fa icon={faSpinner} spin /></div>
	{/if}
</div>
