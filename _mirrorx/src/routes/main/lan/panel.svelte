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
	import { emitNotification } from '$lib/components/notification';
	import { emit } from '@tauri-apps/api/event';

	export let display_name: string;
	export let addrs: Map<string, number>;
	export let os: string;
	export let os_version: string;

	$: ips = Object.keys(addrs);

	const lan_connect = async () => {
		try {
			await emit('/dialog/lan_connect', { addr: ips[0], hostname: display_name });
		} catch (error: any) {
			await emitNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};
</script>

<slot>
	<button
		on:click={lan_connect}
		class="flex flex-row items-center rounded-lg p-2 transition-all hover:rounded-lg hover:bg-primary hover:text-primary-content"
		style="height: 76px"
	>
		<div class="w-full flex-1">
			<div class=" w-48 overflow-hidden text-ellipsis whitespace-nowrap text-left text-lg">
				{display_name}
			</div>
			<div class="w-48 overflow-hidden text-ellipsis whitespace-nowrap text-left text-xs">
				{os}&nbsp;{os_version}
			</div>
			<div class="w-48 text-left text-xs">
				{ips[0]}
			</div>
		</div>

		<div class="flex-0 flex w-14 items-center justify-center text-4xl">
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

		<!-- <div class="flex-0"> -->
		<!-- {#if is_orphan || show_connect_button} -->
		<!-- <div class="btn-group pr-2">
					<button class="btn btn-xs btn-active inline-flex" on:click={connect_lan}>
						<Fa icon={faDisplay} />
						
					</button>

					<button class="btn btn-xs btn-disabled inline-flex">
						<Fa icon={faFolderTree} />
					</button>
				</div> -->
		<!-- {/if} -->
		<!-- </div> -->
	</button>
</slot>
