<script context="module" lang="ts">
</script>

<script lang="ts">
	import type { NotificationEvent } from '$lib/components/notification';
	import { faXmark } from '@fortawesome/free-solid-svg-icons';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import LL from '$lib/i18n/i18n-svelte';
	import { v4 as uuidv4 } from 'uuid';
	import { isMacOS } from '$lib/components/types';

	let show: boolean = false;
	let notification_event: {
		level_color: string;
		text_color: string;
		title: string;
		message: string;
		id: string;
	} | null = null;
	let unlisten_fn: UnlistenFn | null = null;

	onMount(async () => {
		unlisten_fn = await listen<NotificationEvent>('/dialog/notification', (event) => {
			console.log('/dialog/notification: ' + JSON.stringify(event));

			let level_color = '';
			let text_color = '';
			switch (event.payload.level) {
				case 'info':
					level_color = 'bg-info';
					text_color = 'text-info-content';
					break;
				case 'success':
					level_color = 'bg-success';
					text_color = 'text-success-content';
					break;
				case 'warning':
					level_color = 'bg-warning';
					text_color = 'text-warning-content';
					break;
				case 'error':
					level_color = 'bg-error';
					text_color = 'text-error-content';
					break;
			}

			notification_event = {
				level_color: level_color,
				text_color: text_color,
				title: event.payload.title,
				message: event.payload.message,
				id: uuidv4()
			};

			show = true;
		});
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}
	});

	const dismiss = () => {
		show = false;
		notification_event = null;
	};
</script>

<slot>
	<input type="checkbox" id="dialog_notification" class="modal-toggle" checked={show} />
	<div data-tauri-drag-region class="modal {isMacOS ? '' : 'rounded-lg'}">
		<!-- here rounded-lg used for mask layer on windows because windows has css rounded corners-->
		<div class="modal-box {notification_event?.level_color}">
			<h3 class="{notification_event?.text_color} text-lg font-bold">{notification_event?.title}</h3>
			<div class="{notification_event?.text_color} break-words py-4">{notification_event?.message}</div>
			<div class="modal-action">
				<button class="btn" on:click={dismiss}>{$LL.DialogActions.Ok()}</button>
			</div>
		</div>
	</div>
</slot>
