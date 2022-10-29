<script lang="ts">
	import Fa from 'svelte-fa';
	import {
		faPenToSquare,
		faEye,
		faEyeSlash,
		faDisplay,
		faFolderTree,
		faSpinner
	} from '@fortawesome/free-solid-svg-icons';
	import LL, { setLocale } from '../../i18n/i18n-svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	export let domain: String;

	var input_remote_device_id_before: String;
	var input_remote_device_id: String;
	var device_id: String;
	var device_password: String;
	var show_password = false;

	$: {
		load_device_id(domain);
		load_device_password(domain);
	}

	const load_device_id = async (domain: String) => {
		try {
			console.log(domain);
			device_id = await invoke('get_config_device_id', { domain });
		} catch (error) {
			// todo: pop dialog
		}
	};

	const load_device_password = async (domain: String) => {
		try {
			device_password = await invoke('get_config_device_password', { domain });
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
</script>

<slot>
	{#if domain && device_id && device_id}
		<div class="mx-2 flex h-full flex-col">
			<div class="my-3 text-center text-3xl">{$LL.Pages.Connect.DeviceID()}</div>
			<div class="my-3 text-center text-4xl">{device_id}</div>
			<div class="my-3 text-center text-3xl">{$LL.Pages.Connect.Password()}</div>
			<div class="my-3 text-center">
				{#if show_password}
					<p class="text-3xl">{device_password}</p>
				{:else}
					<p class="text-4xl">＊＊＊＊＊＊</p>
				{/if}
				<br />
				<p>
					<button class="tooltip tooltip-bottom text-xl" data-tip={$LL.Pages.Connect.Tooltips.EditPassword()}>
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
					<button class="btn btn-active tooltip inline-flex" data-tip="SSS">
						<Fa class="mr-2" icon={faDisplay} />
						{$LL.Pages.Connect.Desktop()}
					</button>

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
