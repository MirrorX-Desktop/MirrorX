<script lang="ts">
	import '../app.css';
	import Connect from './home/connect.svelte';
	import History from './home/history.svelte';
	import Lan from './home/lan.svelte';

	var select_tab: String = 'connect';

	const switch_tab = (tab_name: String) => (select_tab = tab_name);
</script>

<div class="flex h-full flex-col">
	<div class="flex-none">
		<div class="mt-1 mb-2 text-center text-2xl">Domain</div>
		<div class="my-2 text-center text-4xl">MirrorX.cloud</div>
		<div class="btn-group my-3 mx-2 flex flex-row">
			<button
				class="btn flex-1 {select_tab == 'connect' ? 'btn-active' : undefined}"
				on:click={() => switch_tab('connect')}>Connect</button
			>
			<button class="btn flex-1 {select_tab == 'lan' ? 'btn-active' : undefined}" on:click={() => switch_tab('lan')}
				>LAN</button
			>
			<button
				class="btn flex-1 {select_tab == 'history' ? 'btn-active' : undefined}"
				on:click={() => switch_tab('history')}>History</button
			>
		</div>
	</div>

	<div class="flex-1">
		{#if select_tab == 'connect'}
			<Connect />
		{/if}
		{#if select_tab == 'lan'}
			<Lan />
		{/if}
		{#if select_tab == 'history'}
			<History />
		{/if}
	</div>

	<div class="mx-2 flex-none text-center">
		<hr />
		<a
			class="align-text-top text-xs text-blue-500 hover:text-blue-600"
			href="https://github.com/MirrorX-Desktop/MirrorX">MirrorX</a
		>
	</div>
</div>
