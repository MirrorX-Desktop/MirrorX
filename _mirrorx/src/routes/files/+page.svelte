<script lang="ts">
	import { page } from '$app/stores';
	import {
		invoke_file_manager_send_file,
		invoke_file_manager_visit_local,
		invoke_file_manager_visit_remote
	} from '$lib/components/command';
	import type { Directory, Entry } from '$lib/components/types';
	import { faSpinner } from '@fortawesome/free-solid-svg-icons';
	import { onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import View from './view.svelte';
	import { encode } from 'js-base64';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import Transfer from './transfer.svelte';

	let remote_device_id: string = $page.url.searchParams.get('device_id')!;
</script>

<div class="flex h-full w-full flex-col gap-2 bg-base-100 p-2">
	<div class="flex flex-1 flex-row gap-2 overflow-hidden">
		<View remoteDeviceID={remote_device_id} isLocal={true} />
		<View remoteDeviceID={remote_device_id} isLocal={false} />
	</div>

	<div class="flex-0 h-52 w-full overflow-hidden">
		<Transfer />
	</div>
</div>
