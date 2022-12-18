<script lang="ts">
	import {
		faCheck,
		faXmark,
		faDisplay,
		faEyeSlash,
		faFolderTree,
		faPenToSquare,
		faRotate,
		faSpinner,
		faAsterisk
	} from '@fortawesome/free-solid-svg-icons';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import {
		invoke_utility_generate_random_password,
		invoke_config_domain_update,
		invoke_signaling_visit
	} from '$lib/components/command';
	import { current_domain } from '$lib/components/stores';
	import { onDestroy, onMount } from 'svelte';
	import type { Unsubscriber } from 'svelte/store';
	import LL from '$lib/i18n/i18n-svelte';
	import { emitNotification } from '$lib/components/notification';
	import { writeText, readText } from '@tauri-apps/api/clipboard';
	import type { Domain } from '$lib/components/types';
	import Fa from 'svelte-fa';
	import { formatDeviceID } from '$lib/components/utility';

	let domain: Domain | null = null;
	let domain_unsubscribe: Unsubscriber | null = null;
	let input_remote_device_id_before = '';
	let input_remote_device_id = '';
	let show_password = false;
	let edit_password = false;
	let random_password_generating = false;
	let desktop_is_connecting = false;
	let desktop_is_connecting_unlisten_fn: UnlistenFn | null;
	let domain_id_copied = false;
	let remote_device_id_input: HTMLElement | null = null;
	let remote_device_id_input_placeholder: HTMLElement | null = null;

	$: device_password_display = domain?.password ?? '';

	$: remote_device_valid = input_remote_device_id.length == 0 || /^\d{2}-\d{4}-\d{4}$/.test(input_remote_device_id);

	onMount(async () => {
		domain_unsubscribe = current_domain.subscribe((value) => {
			domain = value;
		});

		desktop_is_connecting_unlisten_fn = await listen<boolean>('desktop_is_connecting', (event) => {
			desktop_is_connecting = event.payload;

			if (!event.payload) {
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

	const on_remote_device_id_input = async (
		event: Event & {
			currentTarget: EventTarget & HTMLInputElement;
		}
	) => {
		let input_event = event as InputEvent & {
			currentTarget: EventTarget & HTMLInputElement;
		};
		console.log(input_event);
		if (input_event.inputType == 'insertFromPaste') {
			// paste device_id from clipboard
			readText().then((v) => {
				let matched_ids = v?.match(/^\d{2}-\d{4}-\d{4}$/);
				if (matched_ids != null && matched_ids.length > 0) {
					input_remote_device_id = matched_ids[0];
				} else {
					input_event.currentTarget.value = '';
				}
			});
		} else if (
			(input_event.inputType == 'deleteContentBackward' || input_event.inputType == 'deleteContentForward') &&
			input_remote_device_id_before.endsWith('-')
		) {
			// delete should keep last '-' until next delete
			input_remote_device_id = input_remote_device_id_before.substring(0, input_remote_device_id_before.length - 2);
		} else {
			let value = input_event.currentTarget.value.replace(/\D/g, '');

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
			device_password_display = await invoke_utility_generate_random_password();
		} catch (error: any) {
			await emitNotification({ level: 'error', title: 'Error', message: error.toString() });
		}

		random_password_generating = false;
	};

	const cancel_edit_password = () => {
		edit_password = false;
		device_password_display = domain?.password ?? '';
	};

	const commit_edit_password = async () => {
		try {
			await invoke_config_domain_update(domain?.id ?? 0, { password: device_password_display });
			edit_password = false;
			if (domain) {
				domain.password = device_password_display;
				current_domain.set(domain);
			}
		} catch (error: any) {
			await emitNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};

	const connect_desktop = async () => {
		try {
			if (!/^\d{2}-\d{4}-\d{4}$/.test(input_remote_device_id)) {
				return;
			}
			await emit('desktop_is_connecting', true);
			await emit('/dialog/visit_prepare', { remote_device_id: input_remote_device_id });
		} catch (error: any) {
			await emit('desktop_is_connecting', false);
			await emitNotification({ level: 'error', title: 'Error', message: error.toString() });
		}
	};

	const copy_domain_id = () => {
		if (domain) {
			writeText(formatDeviceID(domain.device_id));
			domain_id_copied = true;
		}
	};

	const change_remote_device_id_input_placeholder_visible = (visible: boolean) => {
		if (visible) {
			if (remote_device_id_input_placeholder && input_remote_device_id.length == 0) {
				remote_device_id_input_placeholder.classList.remove('invisible');
				remote_device_id_input_placeholder.classList.add('visible');
			}
		} else {
			remote_device_id_input_placeholder?.classList.remove('visible');
			remote_device_id_input_placeholder?.classList.add('invisible');
			remote_device_id_input?.focus();
		}
	};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<slot>
	<div class="flex h-full w-full flex-col p-2">
		<div class="flex h-16 items-center justify-center">
			<div class="text-3xl">
				{$LL.Home.Domain()}
			</div>
		</div>
		<div class=" flex h-16 items-center justify-center text-center">
			{#if domain}
				<div class="text-4xl">{domain.name}</div>
			{:else}
				<Fa class="w-full text-center" icon={faSpinner} spin={true} size={'sm'} />
			{/if}
		</div>

		<div class="flex h-16 items-center justify-center">
			<div class="text-3xl">{$LL.Home.DeviceID()}</div>
		</div>

		<div class="flex h-16 items-center justify-center">
			{#if domain}
				<button
					class="tooltip tooltip-bottom text-4xl hover:cursor-pointer"
					data-tip={domain_id_copied
						? $LL.Home.ClickToCopyDeviceIDCopiedTooltip()
						: $LL.Home.ClickToCopyDeviceIDTooltip()}
					on:click={copy_domain_id}
					on:mouseleave={() => (domain_id_copied = false)}
				>
					{formatDeviceID(domain.device_id)}
				</button>
			{:else}
				<Fa class="w-full text-center" icon={faSpinner} spin={true} size={'sm'} />
			{/if}
		</div>
		<div class="flex h-16 items-center justify-center">
			<div class="text-3xl">{$LL.Home.Password()}</div>
		</div>

		<div class="flex h-16 items-center justify-center">
			{#if edit_password}
				<input
					id="password_input"
					class="input input-bordered ring-info focus:border-info flex-1 text-center focus:outline-none focus:ring"
					type="text"
					placeholder={''}
					maxlength="20"
					value={device_password_display}
					on:change={(event) => (device_password_display = event.currentTarget.value)}
				/>
			{:else if show_password}
				<p class="text-2xl">{device_password_display}</p>
			{:else}
				<Fa icon={faAsterisk} class="px-1" />
				<Fa icon={faAsterisk} class="px-1" />
				<Fa icon={faAsterisk} class="px-1" />
				<Fa icon={faAsterisk} class="px-1" />
				<Fa icon={faAsterisk} class="px-1" />
				<Fa icon={faAsterisk} class="px-1" />
				<Fa icon={faAsterisk} class="px-1" />
			{/if}
		</div>

		<div class="flex h-8 items-center justify-center gap-3">
			{#if edit_password}
				<button
					class="tooltip tooltip-bottom text-xl"
					data-tip={$LL.Home.GenerateRandomPasswordTooltip()}
					on:click={generate_random_password}
				>
					<Fa icon={faRotate} spin={random_password_generating} />
				</button>

				<button
					class="tooltip tooltip-bottom text-xl"
					data-tip={$LL.Home.EditPasswordTooltip()}
					on:click={commit_edit_password}
				>
					<Fa icon={faCheck} />
				</button>

				<button
					class="tooltip tooltip-bottom text-xl"
					data-tip={$LL.Home.EditPasswordCancelTooltip()}
					on:click={cancel_edit_password}
				>
					<Fa icon={faXmark} />
				</button>
			{:else}
				<button
					class="tooltip tooltip-bottom text-xl"
					data-tip={$LL.Home.EditPasswordTooltip()}
					on:click={() => (edit_password = true)}
				>
					<Fa icon={faPenToSquare} />
				</button>

				<button
					class="tooltip tooltip-bottom text-xl"
					data-tip={show_password ? $LL.Home.PasswordInvisibleTooltip() : $LL.Home.PasswordVisibleTooltip()}
					on:click={() => (show_password = true)}
					on:mouseleave={() => (show_password = false)}
				>
					<Fa icon={faEyeSlash} />
				</button>
			{/if}
		</div>
		<div class="divider mb-2">{$LL.Home.Connect()}</div>
		<div class="flex h-full flex-1 flex-row items-center justify-center">
			<div class="relative w-5/6">
				<input
					class="input input-bordered w-full  text-center text-3xl {remote_device_valid
						? 'ring-info focus:border-info focus:outline-none focus:ring'
						: 'ring-error focus:border-error focus:outline-none focus:ring'}"
					type="text"
					maxlength="12"
					bind:this={remote_device_id_input}
					bind:value={input_remote_device_id}
					on:blur={() => change_remote_device_id_input_placeholder_visible(true)}
					on:beforeinput={(ev) => (input_remote_device_id_before = ev.currentTarget.value)}
					on:input={(event) => on_remote_device_id_input(event)}
				/>

				<div
					bind:this={remote_device_id_input_placeholder}
					on:click={() => change_remote_device_id_input_placeholder_visible(false)}
					class="absolute top-0 h-full w-full cursor-text text-center align-middle text-lg"
					style="line-height: 48px;"
				>
					{$LL.Home.RemoteDeviceID()}
				</div>
			</div>
		</div>
		<div class="flex flex-row items-center justify-center pb-2">
			<div class="btn-group">
				{#if desktop_is_connecting}
					<button class="btn btn-active btn-disabled">
						<Fa icon={faSpinner} spin />
					</button>
				{:else}
					<button class="btn btn-active inline-flex" on:click={connect_desktop}>
						<Fa class="mr-2" icon={faDisplay} />
						{$LL.Home.Desktop()}
					</button>
				{/if}

				<button class="btn inline-flex">
					<Fa class="mr-2" icon={faFolderTree} />{$LL.Home.Files()}
				</button>
			</div>
		</div>
	</div>
</slot>

<style>
</style>
