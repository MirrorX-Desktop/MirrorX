<script lang="ts">
	import LL from '$lib/i18n/i18n-svelte';
	import { faSpinner } from '@fortawesome/free-solid-svg-icons';
	import { getVersion, getTauriVersion } from '@tauri-apps/api/app';
	import { onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import icon from '../../../../src-tauri/assets/icons/icon.png';

	let version: string | null = null;
	let tauriVersion: string | null = null;

	onMount(async () => {
		version = await getVersion();
		tauriVersion = await getTauriVersion();
	});
</script>

<slot>
	<div class="mx-2 flex h-full flex-row items-center justify-center">
		<div class="text-center">
			<div class="my-4 flex flex-row justify-center">
				<img src={icon} alt="MirrorX Icon" width="128" height="128" />
			</div>
			<div class="my-4 text-4xl">MirrorX</div>
			<div class="my-4 flex items-center justify-evenly text-2xl">
				{$LL.Settings.Pages.About.Version()}&nbsp;&nbsp;
				<span>
					{#if version != null}
						v{version}
					{:else}
						<Fa icon={faSpinner} spin />
					{/if}
				</span>
			</div>
			<div class="my-4 flex items-center justify-evenly text-2xl">
				Tauri&nbsp;&nbsp;
				<span>
					{#if tauriVersion != null}
						v{tauriVersion}
					{:else}
						<Fa icon={faSpinner} spin />
					{/if}
				</span>
			</div>
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
