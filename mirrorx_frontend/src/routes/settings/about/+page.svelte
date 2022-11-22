<script lang="ts">
	import LL from '$lib/i18n/i18n-svelte';
</script>

<slot>
	<div class="mx-2 flex h-full flex-row items-center justify-center">
		<div class="text-center">
			<div class="my-4">
				<div class="avatar placeholder">
					<div class="bg-neutral-focus text-neutral-content w-24 rounded-xl">
						<span class="text-3xl">M</span>
					</div>
				</div>
			</div>
			<div class="my-4 text-4xl">MirrorX</div>
			<div class="my-4 text-2xl">{$LL.Settings.Pages.About.Version()}&nbsp;&nbsp;v0.1.1</div>
			<div class="my-4">
				<a class="mx-2 text-blue-500 hover:text-blue-600" href="https://mirrorx.cloud" target="_blank" rel="noreferrer">
					{$LL.Settings.Pages.About.Official()}
				</a>
				<a
					class="mx-2 text-blue-500 hover:text-blue-600"
					href="https://github.com/MirrorX-Desktop/MirrorX"
					target="_blank"
					rel="noreferrer"
				>
					{$LL.Settings.Pages.About.SourceRepository()}
				</a>
				<a
					class="mx-2 text-blue-500 hover:text-blue-600"
					href="https://github.com/MirrorX-Desktop/MirrorX/issues"
					target="_blank"
					rel="noreferrer"
				>
					{$LL.Settings.Pages.About.SupportAndHelp()}
				</a>
			</div>
		</div>
	</div>
</slot>
