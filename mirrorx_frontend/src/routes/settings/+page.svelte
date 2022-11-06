<script lang="ts">
	import About from './about.svelte';
	import Domain from './domain.svelte';

	let select_tab = 'domain';
	let tabs = [
		{
			name: 'domain',
			component: Domain,
			title: 'Domain'
		},
		{
			name: 'about',
			component: About,
			title: 'About'
		}
	];
</script>

<div class="flex h-full w-full flex-row ">
	<div class="h-full border-r">
		<ul class="menu bg-base-100 w-56">
			{#each tabs as tab}
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<li on:click={() => (select_tab = tab.name)}>
					<span class={select_tab == tab.name ? 'active' : ''}>{tab.title}</span>
				</li>
			{/each}
		</ul>
	</div>

	<div class="h-full flex-1">
		{#if select_tab == 'domain'}
			<Domain />
		{/if}

		{#if select_tab == 'about'}
			<About />
		{/if}
	</div>
</div>
