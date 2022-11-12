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
	import {
		invoke_generate_random_password,
		invoke_set_current_domain_device_password,
		invoke_signaling_visit_request
	} from '../../../components/command';
	import { current_domain, type CurrentDomain } from '../../../components/stores';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import type { Unsubscriber } from 'svelte/store';
	import LL from '../../../i18n/i18n-svelte';
	import { emitHomeNotification } from '../home_notification_center.svelte';

	let domain: CurrentDomain | null = null;
	let domain_unsubscribe: Unsubscriber | null = null;
	let input_remote_device_id_before: string = '';
	let input_remote_device_id: string = '';
	let show_password = false;
	let edit_password = false;
	let random_password_generating = false;
	let desktop_is_connecting = false;
	let desktop_is_connecting_unlisten_fn: UnlistenFn | null;

	$: device_password_display = domain?.password ?? '';
	$: remote_device_valid = input_remote_device_id.length == 0 || /^\d{2}-\d{4}-\d{4}$/.test(input_remote_device_id);

	onMount(async () => {
		domain_unsubscribe = current_domain.subscribe((value) => {
			domain = value;
		});

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
		if (domain_unsubscribe) {
			domain_unsubscribe();
		}

		if (desktop_is_connecting_unlisten_fn) {
			desktop_is_connecting_unlisten_fn();
		}
	});

	const on_remote_device_id_input = (
		event: Event & {
			currentTarget: EventTarget & HTMLInputElement;
		}
	) => {
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
	};

	const generate_random_password = async () => {
		try {
			random_password_generating = true;
			device_password_display = await invoke_generate_random_password();
		} catch (error: any) {
			emitHomeNotification({ level: 'error', title: 'Error', message: error.toString() });
		}

		random_password_generating = false;
	};

	const cancel_edit_password = () => {
		edit_password = false;
		device_password_display = domain?.password ?? '';
	};

	const commit_edit_password = async () => {
		try {
			await invoke_set_current_domain_device_password({ password: device_password_display });
			edit_password = false;
			if (domain) {
				domain.password = device_password_display;
				current_domain.set(domain);
			}
		} catch (error: any) {
			emitHomeNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};

	const connect_desktop = async () => {
		try {
			if (!/^\d{2}-\d{4}-\d{4}$/.test(input_remote_device_id)) {
				return;
			}
			await emit('desktop_is_connecting', true);
			await invoke_signaling_visit_request({ remoteDeviceId: input_remote_device_id });
		} catch (error: any) {
			await emit('desktop_is_connecting', false);
			emitHomeNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};
</script>

<slot>
	{#if domain && domain.device_id && domain.name && domain.password}
		<div class="mx-2 flex h-full flex-col">
			<div class="my-3 text-center text-3xl">{$LL.Home.Pages.Connect.DeviceID()}</div>
			<div class="my-3 text-center text-4xl">{domain.device_id}</div>
			<div class="my-3 text-center text-3xl">{$LL.Home.Pages.Connect.Password()}</div>
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
							value={device_password_display}
							on:change={(event) => (device_password_display = event.currentTarget.value)}
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
							data-tip={$LL.Home.Pages.Connect.Tooltips.EditPasswordCancel()}
							on:click={cancel_edit_password}
						>
							<Fa icon={faCircleXmark} />
						</button>
					{:else}
						<button
							class="tooltip tooltip-bottom text-xl"
							data-tip={$LL.Home.Pages.Connect.Tooltips.EditPassword()}
							on:click={() => (edit_password = true)}
						>
							<Fa icon={faPenToSquare} />
						</button>
						<button
							class="tooltip tooltip-bottom text-xl"
							data-tip={show_password
								? $LL.Home.Pages.Connect.Tooltips.PasswordInvisible()
								: $LL.Home.Pages.Connect.Tooltips.PasswordVisible()}
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
					class="w-5/6 rounded border p-2 text-center text-3xl {remote_device_valid
						? 'ring-blue-400 focus:outline-none focus:ring'
						: 'outline-none ring ring-red-500'}"
					type="text"
					placeholder={$LL.Home.Pages.Connect.RemoteDeviceIDPlaceHolder()}
					maxlength="12"
					bind:value={input_remote_device_id}
					on:beforeinput={(ev) => (input_remote_device_id_before = ev.currentTarget.value)}
					on:input={(event) => on_remote_device_id_input(event)}
				/>
				<div class="btn-group">
					{#if desktop_is_connecting}
						<button class="btn btn-active btn-disabled">
							<Fa icon={faSpinner} spin />
						</button>
					{:else}
						<button class="btn btn-active inline-flex" on:click={connect_desktop}>
							<Fa class="mr-2" icon={faDisplay} />
							{$LL.Home.Pages.Connect.Desktop()}
						</button>
					{/if}

					<button class="btn inline-flex">
						<Fa class="mr-2" icon={faFolderTree} />{$LL.Home.Pages.Connect.Files()}
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
	/* #remote_device_id_input::-webkit-input-placeholder {
		@apply text-center align-middle text-xl;
	}

	#remote_device_id_input::placeholder {
		@apply text-center align-middle text-xl;
	}

	#remote_device_id_input::-moz-placeholder {
		@apply text-center align-middle text-xl;
	}

	#remote_device_id_input::-ms-input-placeholder {
		@apply text-center align-middle text-xl;
	} */
</style>
