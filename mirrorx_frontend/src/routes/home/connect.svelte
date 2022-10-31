<script lang="ts">
	import {
		faCheck,
		faCircleXmark,
		faDisplay,
		faEyeSlash,
		faFolderTree,
		faPenToSquare,
		faRotate,
		faSpinner
	} from '@fortawesome/free-solid-svg-icons';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import LL from '../../i18n/i18n-svelte';

	export let domain: string;

	var input_remote_device_id_before: string;
	var input_remote_device_id: string;
	var device_id: string;
	var device_password: string;
	var device_password_display: string;
	var show_password = false;
	var edit_password = false;
	var random_password_generating = false;
	let desktop_is_connecting = false;
	var desktop_is_connecting_unlisten_fn: UnlistenFn | null;

	$: {
		load_device_id(domain);
		load_device_password(domain);
	}

	onMount(async () => {
		desktop_is_connecting_unlisten_fn = await listen<string>('desktop_is_connecting', (event) => {
			let is_connecting: boolean = JSON.parse(event.payload);
			desktop_is_connecting = is_connecting;

			if (!is_connecting) {
				input_remote_device_id = '';
				input_remote_device_id_before = '';
			}
		});
	});

	onDestroy(() => {
		if (desktop_is_connecting_unlisten_fn) {
			desktop_is_connecting_unlisten_fn();
		}
	});

	const load_device_id = async (domain: string) => {
		try {
			console.log(domain);
			device_id = await invoke('get_config_device_id', { domain });
		} catch (error) {
			// todo: pop dialog
		}
	};

	const load_device_password = async (domain: string) => {
		try {
			device_password = await invoke('get_config_device_password', { domain });
			device_password_display = device_password;
		} catch (error) {
			// todo: pop dialog
		}
	};

	function on_remote_device_id_input(
		event: Event & {
			currentTarget: EventTarget & HTMLInputElement;
		}
	) {
		let input_event = event as InputEvent & {
			currentTarget: EventTarget & HTMLInputElement;
		};

		if (input_event.inputType == 'insertFromPaste') {
			// paste device_id from clipboard

			let matched_ids = input_event.data?.match(/^\d{2}-\d{4}-\d{4}$/g);

			if (matched_ids != null && matched_ids.length >= 1) {
				input_remote_device_id = matched_ids[0];
			} else {
				input_event.currentTarget.value = '';
			}
		} else if (
			(input_event.inputType == 'deleteContentBackward' || input_event.inputType == 'deleteContentForward') &&
			input_remote_device_id_before.endsWith('-')
		) {
			// delete should keep last '-' until next delete
			input_remote_device_id = input_remote_device_id_before.substring(0, input_remote_device_id_before.length - 2);
		} else {
			var value = input_event.currentTarget.value.replace(/[^\d]/g, '');

			if (value.length >= 6) {
				value = value.substring(0, 2) + '-' + value.substring(2, 6) + '-' + value.substring(6);
			} else if (value.length >= 2) {
				value = value.substring(0, 2) + '-' + value.substring(2);
			}

			input_remote_device_id = value;
		}
	}

	const generate_random_password = async () => {
		try {
			random_password_generating = true;
			device_password_display = await invoke('generate_random_password');
		} catch {
			// todo: pop dialog
		}

		random_password_generating = false;
	};

	const cancel_edit_password = () => {
		edit_password = false;
		device_password_display = device_password;
	};

	const commit_edit_password = async () => {
		try {
			await invoke('set_config_device_password', { domain, password: device_password_display });
			await load_device_password(domain);
			edit_password = false;
			device_password_display = device_password;
		} catch (error) {
			// todo: pop dialog
		}
	};

	const connect_desktop = async () => {
		try {
			emit('desktop_is_connecting', true);
			await invoke('signaling_visit_request', { domain, remoteDeviceId: input_remote_device_id });
		} catch (error) {
			emit('desktop_is_connecting', false);
			console.log('visit request: ' + error);
			// todo: pop dialog
		}
	};
</script>

<slot>
	{#if domain && device_id && device_id}
		<div class="mx-2 flex h-full flex-col">
			<div class="my-3 text-center text-3xl">{$LL.Pages.Connect.DeviceID()}</div>
			<div class="my-3 text-center text-4xl">{device_id}</div>
			<div class="my-3 text-center text-3xl">{$LL.Pages.Connect.Password()}</div>
			<div class="my-3 text-center">
				{#if edit_password}
					<div class="input-group flex flex-row">
						<button class="btn btn-square flex-none" on:click={generate_random_password}>
							<Fa icon={faRotate} spin={random_password_generating} />
						</button>

						<input
							id="remote_device_id_input"
							class="input input-bordered flex-1 text-center focus:border-blue-300 focus:outline-none focus:ring"
							type="text"
							placeholder={''}
							maxlength="20"
							bind:value={device_password_display}
						/>

						<button class="btn btn-square flex-none" on:click={commit_edit_password}>
							<Fa icon={faCheck} />
						</button>
					</div>
				{:else if show_password}
					<p class="text-3xl">{device_password_display}</p>
				{:else}
					<p class="text-4xl">＊＊＊＊＊＊</p>
				{/if}

				<br />

				<p>
					{#if edit_password}
						<button
							class="tooltip tooltip-bottom text-xl"
							data-tip={$LL.Pages.Connect.Tooltips.EditPassword()}
							on:click={cancel_edit_password}
						>
							<Fa icon={faCircleXmark} />
						</button>
					{:else}
						<button
							class="tooltip tooltip-bottom text-xl"
							data-tip={$LL.Pages.Connect.Tooltips.EditPassword()}
							on:click={() => (edit_password = true)}
						>
							<Fa icon={faPenToSquare} />
						</button>
						<button
							class="tooltip tooltip-bottom text-xl"
							data-tip={show_password
								? $LL.Pages.Connect.Tooltips.PasswordInvisible()
								: $LL.Pages.Connect.Tooltips.PasswordVisible()}
							on:click={() => (show_password = true)}
							on:mouseleave={() => (show_password = false)}
						>
							<Fa icon={faEyeSlash} />
						</button>
					{/if}
				</p>
			</div>

			<hr />
			<div class="flex h-full flex-1 flex-col place-items-center justify-evenly">
				<input
					id="remote_device_id_input"
					class="w-5/6 rounded border text-center text-4xl focus:border-blue-300 focus:outline-none focus:ring"
					type="text"
					placeholder={$LL.Pages.Connect.RemoteDeviceIDPlaceHolder()}
					maxlength="12"
					bind:value={input_remote_device_id}
					on:beforeinput={(ev) => (input_remote_device_id_before = ev.currentTarget.value)}
					on:input={(event) => on_remote_device_id_input(event)}
				/>
				<div class="btn-group">
					{#if desktop_is_connecting}
						<button class="btn btn-active tooltip tooltip-bottom btn-disabled" data-tip="SSS">
							<Fa icon={faSpinner} spin />
						</button>
					{:else}
						<button class="btn btn-active tooltip tooltip-bottom inline-flex" data-tip="SSS" on:click={connect_desktop}>
							<Fa class="mr-2" icon={faDisplay} />
							{$LL.Pages.Connect.Desktop()}
						</button>
					{/if}

					<button class="btn tooltip inline-flex" data-tip="AAA">
						<Fa class="mr-2" icon={faFolderTree} />{$LL.Pages.Connect.Files()}
					</button>
				</div>
			</div>
		</div>
	{:else}
		<div class="align-center flex h-full flex-col place-items-center justify-center">
			<div class="flex-none"><Fa icon={faSpinner} spin={true} size={'2x'} /></div>
		</div>
	{/if}
</slot>

<style>
	#remote_device_id_input::-webkit-input-placeholder {
		font-size: large;
	}
</style>
