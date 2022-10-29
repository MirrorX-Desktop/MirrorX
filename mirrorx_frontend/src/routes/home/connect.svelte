<script lang="ts">
	import Fa from 'svelte-fa';
	import { faPenToSquare, faEye, faEyeSlash, faDisplay, faFolderTree } from '@fortawesome/free-solid-svg-icons';
	import { prevent_default } from 'svelte/internal';

	let input_remote_device_id_before: String;
	let input_remote_device_id: String;

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
	<div class="mx-2 flex h-full flex-col">
		<div class="mb-4 text-center text-3xl">Device ID</div>
		<div class="my-4 text-center text-4xl">12-3456-7890</div>
		<div class="my-4 text-center text-3xl">Password</div>
		<div class="my-4 text-center text-4xl">
			＊＊＊＊＊＊
			<p>
				<button class="text-xl"><Fa icon={faPenToSquare} /></button>
				<button class="text-xl"><Fa icon={faEyeSlash} /></button>
			</p>
		</div>
		<hr />
		<div class="flex h-full flex-1 flex-col place-items-center justify-evenly">
			<input
				id="remote_device_id_input"
				class="w-5/6 rounded border text-center text-4xl focus:border-blue-300 focus:outline-none focus:ring"
				type="text"
				placeholder="Remote Device ID"
				maxlength="12"
				bind:value={input_remote_device_id}
				on:beforeinput={(ev) => (input_remote_device_id_before = ev.currentTarget.value)}
				on:input={(event) => on_remote_device_id_input(event)}
			/>
			<div class="btn-group">
				<button class="btn btn-active"><Fa class="m-1" icon={faDisplay} />Desktop</button>
				<button class="btn"><Fa class="m-1" icon={faFolderTree} />Files</button>
			</div>
		</div>
	</div>
</slot>

<style>
	#remote_device_id_input::-webkit-input-placeholder {
		font-size: large;
	}
</style>
