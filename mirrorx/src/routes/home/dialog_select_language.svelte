<script lang="ts">
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { faXmark } from '@fortawesome/free-solid-svg-icons';
	import LL from '$lib/i18n/i18n-svelte';
	import { emitHomeNotification } from './notification_home.svelte';
	import { invoke_config_language_get, invoke_config_language_set } from '$lib/components/command';

	let show = false;
	let language = '';
	let unlisten_fn: UnlistenFn | null = null;

	$: {
		(async function () {
			if (language && language.length > 0) {
				try {
					await invoke_config_language_set(language);
				} catch (error: any) {
					await emitHomeNotification({
						level: 'error',
						title: 'Error',
						message: error.toString()
					});
				} finally {
					show = false;
				}
			}
		})();
	}

	const localeAndDisplayNames: Array<{ code: string; name: string }> = [
		{ code: 'en', name: 'English' },
		{ code: 'zh', name: '中文' }
	];

	onMount(async () => {
		unlisten_fn = await listen<string>('home:show_select_language_dialog', (event) => {
			show = true;
		});

		invoke_config_language_get().then((v) => (language = v));
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}
	});
</script>

<slot>
	<input type="checkbox" id="dialog_select_language" class="modal-toggle" checked={show} />
	<div class="modal">
		<div class="modal-box w-80">
			<button on:click={() => (show = false)} class="btn btn-xs btn-circle btn-outline absolute right-2 top-2">
				<Fa icon={faXmark} />
			</button>
			<h3 class="text-lg font-bold">{$LL.Home.Layout.Dialog.SelectLanguage.Title()}</h3>
			<div class="py-2">
				{#each localeAndDisplayNames as ld}
					<div class="form-control">
						<label class="label cursor-pointer">
							<span class="label-text">{ld.name}</span>
							<input type="radio" bind:group={language} name="languages" class="radio" value={ld.code} />
						</label>
					</div>
				{/each}
			</div>
		</div>
	</div>
</slot>
