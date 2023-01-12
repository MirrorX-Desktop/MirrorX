<script lang="ts">
	import { page } from '$app/stores';
	import { invoke_file_manager_send_file, invoke_file_manager_visit_local, invoke_file_manager_visit_remote } from '$lib/components/command';
	import type { Directory, Entry } from '$lib/components/types';
	import { faSpinner } from '@fortawesome/free-solid-svg-icons';
	import { onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import View from './view.svelte';
	import { encode } from 'js-base64';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';

	let remote_device_id: string = $page.url.searchParams.get('device_id')!;

	let send_file_to_remote_unlisten_fn: UnlistenFn | null = null;
	let download_file_to_local_unlisten_fn: UnlistenFn | null = null;

	onMount(async () => {
		send_file_to_remote_unlisten_fn = await listen<{ entry: Entry; path: string }>(
			'send_file_to_remote',
			async (event) => {
				console.log("begin send "+event.payload.entry);
				await invoke_file_manager_send_file(remote_device_id,event.payload.entry.path,event.payload.path);
				console.log("finish send remote");
			}
		);

		download_file_to_local_unlisten_fn = await listen<{ localPath: string; remotePath: string }>(
			'download_file_to_local',
			(event) => {}
		);
	});
</script>

<div class="flex h-full w-full flex-row justify-between">
	<div class="flex h-full flex-1 flex-row items-center justify-center py-2 pl-2" style="max-width: 49%;min-width: 49%">
		<View remoteDeviceID={null} />
	</div>
	<div class="divider divider-horizontal mx-0.5 w-2">·</div>
	<div class="flex h-full flex-1 flex-row items-center justify-center py-2 pr-2" style="max-width: 49%;min-width: 49%">
		<View remoteDeviceID={remote_device_id} />
	</div>
</div>
