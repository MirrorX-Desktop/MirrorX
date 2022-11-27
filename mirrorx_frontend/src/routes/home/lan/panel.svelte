<script lang="ts">
	import Fa from 'svelte-fa';
	import {
		faApple,
		faWindows,
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

	export let host_name: string;
	export let addr: string;
	export let os: string;
	export let os_version: string;
	export let tcp_port: number;
	export let udp_port: number;
</script>

<slot>
	<div class="flex w-full flex-row">
		<div class="avatar placeholder">
			<div class="bg-neutral-focus text-neutral-content w-16 rounded">
				{#if os == 'macOS'}
					<Fa icon={faApple} />
				{:else if os == 'Windows'}
					<Fa icon={faWindows} />
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
		</div>
		<div class="flex flex-1 flex-col">
			<div>{host_name}</div>
			<div>{os_version}</div>
			<div>{addr}</div>
			<div>
				{#if tcp_port > 0 && udp_port > 0}
					{'TCP/UDP'}
				{:else if tcp_port > 0}
					{'TCP'}
				{:else if udp_port > 0}
					{'UDP'}
				{/if}
			</div>
		</div>
		<div>
			<button class="button">Connect</button>
		</div>
	</div>
</slot>
