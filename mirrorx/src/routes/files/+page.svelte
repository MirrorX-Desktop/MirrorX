<script lang="ts">
	import { page } from '$app/stores';
	import { invoke_file_manager_visit_local, invoke_file_manager_visit_remote } from '$lib/components/command';
	import type { Directory } from '$lib/components/types';
	import { faSpinner } from '@fortawesome/free-solid-svg-icons';
	import { onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import DirView from './dir_view.svelte';
	import { encode } from 'js-base64';

	let remote_device_id: string = $page.url.searchParams.get('device_id')!;
	let currentLocalDirectory: Directory | null = null;
	let currentRemoteDirectory: Directory | null = null;

	onMount(async () => {
		request_remote_dir(null);
		request_local_dir(null);
	});

	const request_remote_dir = async (path: string | null) => {
		console.log('request remote dir: ' + path);
		currentRemoteDirectory = await invoke_file_manager_visit_remote(remote_device_id, path);
	};

	const request_local_dir = async (path: string | null) => {
		console.log('request local dir: ' + path);
		currentLocalDirectory = await invoke_file_manager_visit_local(path);
	};
</script>

<div class="flex h-full w-full flex-row">
	{#if currentLocalDirectory}
		<div class="h-full flex-1">
			<DirView directory={currentLocalDirectory} clickItem={(path) => request_local_dir(path)} />
		</div>
	{:else}
		<div class="flex h-full w-full flex-row items-center justify-center"><Fa icon={faSpinner} spin /></div>
	{/if}
	<div class="divider divider-horizontal mx-0.5 w-2">·</div>
	{#if currentRemoteDirectory}
		<div class="h-full flex-1">
			<DirView directory={currentRemoteDirectory} clickItem={(path) => request_remote_dir(path)} />
		</div>
	{:else}
		<div class="flex h-full w-full flex-row items-center justify-center"><Fa icon={faSpinner} spin /></div>
	{/if}
</div>
