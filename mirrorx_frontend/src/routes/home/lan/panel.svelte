<script lang="ts">
	import Fa from 'svelte-fa';
	import { faApple, faWindows, faLinux } from '@fortawesome/free-brands-svg-icons';

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
