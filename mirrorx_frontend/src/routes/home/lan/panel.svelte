<script lang="ts">
	import Fa from 'svelte-fa';
	import {
		faApple,
		faMicrosoft,
		faLinux,
		faUbuntu,
		faFedora,
		faCentos,
		faAndroid,
		faRedhat,
		faSuse,
		faFreebsd,
		faAmazon
	} from '@fortawesome/free-brands-svg-icons';
	import {
		faDisplay,
		faFolderTree,
		faSpinner,
		faNetworkWired,
		faDiagramProject
	} from '@fortawesome/free-solid-svg-icons';
	import { emitHomeNotification } from '../home_notification_center.svelte';
	import { invoke_lan_connect } from '$lib/components/command';

	export let is_orphan: boolean;
	export let host_name: string;
	export let addr: string;
	export let os: string;
	export let os_version: string;

	let show_connect_button: boolean = false;

	const connect_lan = async () => {
		try {
			await invoke_lan_connect({ addr });
		} catch (error: any) {
			await emitHomeNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};
</script>

<slot>
	<tbody on:mouseenter={() => (show_connect_button = true)} on:mouseleave={() => (show_connect_button = false)}>
		<tr>
			<td rowspan="3" class="w-12 text-4xl">
				<div class="flex h-full w-full justify-center">
					{#if os == 'macOS'}
						<Fa icon={faApple} />
					{:else if os == 'Windows'}
						<Fa icon={faMicrosoft} />
					{:else if os == 'Linux'}
						<Fa icon={faLinux} />
					{:else if os == 'Ubuntu'}
						<Fa icon={faUbuntu} />
					{:else if os == 'Fedora'}
						<Fa icon={faFedora} />
					{:else if os == 'CentOS'}
						<Fa icon={faCentos} />
					{:else if os == 'Android'}
						<Fa icon={faAndroid} />
					{:else if os == 'Redhat' || os == 'Redhat Enterprise'}
						<Fa icon={faRedhat} />
					{:else if os == 'SUSE' || os == 'openSUSE'}
						<Fa icon={faSuse} />
					{:else if os == 'FreeBSD'}
						<Fa icon={faFreebsd} />
					{:else if os == 'Amazon'}
						<Fa icon={faAmazon} />
					{/if}
				</div>
			</td>
			<td class="max-w-0 overflow-hidden text-ellipsis whitespace-nowrap pl-2 pr-2 text-xl" colspan="2">{host_name}</td>
			<td rowspan="3" class="w-1">
				{#if is_orphan || show_connect_button}
					<div class="btn-group">
						<!-- {#if desktop_is_connecting} -->
						<!-- <button class="btn btn-active btn-disabled">
					<Fa icon={faSpinner} spin />
				</button> -->
						<!-- {:else} -->
						<button class="btn btn-xs btn-active inline-flex" on:click={connect_lan}>
							<Fa icon={faDisplay} />
							<!-- {$LL.Home.Pages.Connect.Desktop()} -->
						</button>
						<!-- {/if} -->

						<button class="btn btn-xs btn-disabled inline-flex">
							<Fa icon={faFolderTree} />
						</button>
					</div>
				{/if}
			</td>
		</tr>
		<tr>
			<td class="max-w-0 overflow-hidden text-ellipsis whitespace-nowrap pl-2 text-sm text-gray-400" colspan="2"
				>{os}&nbsp;{os_version}</td
			>
		</tr>
		<tr class="text-sm text-gray-400">
			<td>
				<div class="inline-flex place-content-center items-center gap-1 pl-2">
					<Fa icon={faNetworkWired} />
					{addr}
				</div>
			</td>
		</tr>
	</tbody>
</slot>
