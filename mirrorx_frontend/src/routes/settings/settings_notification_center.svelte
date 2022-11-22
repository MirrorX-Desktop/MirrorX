<script context="module" lang="ts">
	import { emit } from '@tauri-apps/api/event';

	export interface SettingsNotificationEvent {
		level: 'info' | 'success' | 'warning' | 'error';
		message: string;
	}

	export async function emitSettingsNotification(notification: SettingsNotificationEvent) {
		await emit('settings_notification', notification);
	}
</script>

<script lang="ts">
	import { faXmark } from '@fortawesome/free-solid-svg-icons';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { v4 as uuidv4 } from 'uuid';

	let notifications: Array<{ level_color: string; message: string; id: string }> = [];
	let unlisten_fn: UnlistenFn | null = null;
	$: notifications_reverse = notifications.reverse().slice(0, 2);

	onMount(async () => {
		unlisten_fn = await listen<SettingsNotificationEvent>('settings_notification', (event) => {
			let level_color: string = '';
			switch (event.payload.level) {
				case 'info':
					level_color = 'alert-info';
				case 'success':
					level_color = 'alert-success';
				case 'warning':
					level_color = 'alert-warning';
				case 'error':
					level_color = 'alert-error';
			}

			notifications.push({
				level_color: level_color,
				message: event.payload.message,
				id: uuidv4()
			});

			notifications = notifications;
		});
	});

	onDestroy(() => {
		if (unlisten_fn) {
			unlisten_fn();
		}
	});
</script>

<slot>
	{#if notifications.length > 0}
		<div class="toast toast-top toast-center w-full">
			{#each notifications_reverse as notification}
				<div class="alert {notification.level_color} shadow-lg">
					<div>
						{#if notification.level_color == 'alert-info'}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								class="h-6 w-6 flex-shrink-0 stroke-current"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
								/></svg
							>
						{:else if notification.level_color == 'alert-success'}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-6 w-6 flex-shrink-0 stroke-current"
								fill="none"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
								/></svg
							>
						{:else if notification.level_color == 'alert-warning'}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-6 w-6 flex-shrink-0 stroke-current"
								fill="none"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
								/></svg
							>
						{:else if notification.level_color == 'alert-error'}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-6 w-6 flex-shrink-0 stroke-current"
								fill="none"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
								/></svg
							>
						{/if}
						<span>{notification.message}</span>
					</div>
					<div class="flex-none">
						<button
							class="btn btn-sm"
							on:click={() => {
								let index = notifications.findIndex((x) => x.id == notification.id);
								if (index != -1) {
									notifications.splice(index, 1);
									notifications = notifications;
								}
							}}>Ok</button
						>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</slot>
