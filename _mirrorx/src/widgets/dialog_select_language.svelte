<script lang="ts">
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { faXmark } from '@fortawesome/free-solid-svg-icons';
	import { faCircle, faCircleDot } from '@fortawesome/free-regular-svg-icons';
	import LL, { locale, setLocale } from '$lib/i18n/i18n-svelte';
	import { emitNotification } from '$lib/components/notification';
	import { invoke_config_language_get, invoke_config_language_set } from '$lib/components/command';
	import { isMacOS } from '$lib/components/types';
	import type { Locales } from '$lib/i18n/i18n-types';

	let show = false;
	let language = '';
	let unlisten_fn: UnlistenFn | null = null;

	const localeAndDisplayNames: Array<{ code: string; name: string }> = [
		{ code: 'en', name: 'English' },
		{ code: 'zh', name: '中文' }
	];

	onMount(async () => {
		unlisten_fn = await listen<string>('/dialog/select_language', (event) => {
			language = $locale;
			show = true;
		});
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}
	});

	const set_language = async (lang: string) => {
		try {
			language = lang;
			await invoke_config_language_set(lang);
			setLocale(lang as Locales);
		} catch (error: any) {
			await emitNotification({
				level: 'error',
				title: 'Error',
				message: error.toString()
			});
		} finally {
			show = false;
		}
	};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<slot>
	<input type="checkbox" id="dialog_select_language" class="modal-toggle" checked={show} />
	<div data-tauri-drag-region class="modal {isMacOS ? '' : 'rounded-lg'}">
		<div class="modal-box w-80">
			<button on:click={() => (show = false)} class="btn btn-xs btn-circle btn-outline absolute right-2 top-2">
				<Fa icon={faXmark} />
			</button>
			<h3 class="text-lg font-bold">{$LL.Dialogs.SelectLanguage.Title()}</h3>
			<div class="py-2">
				{#each localeAndDisplayNames as ld}
					<div
						class="hover:bg-primary hover:text-primary-content flex cursor-pointer flex-row items-center justify-between rounded-lg p-2 transition hover:rounded-lg"
						on:click={() => set_language(ld.code)}
					>
						<div class="text-lg">{ld.name}</div>
						{#if ld.code == $locale}
							<Fa icon={faCircleDot} />
						{:else}
							<Fa icon={faCircle} />
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</div>
</slot>
