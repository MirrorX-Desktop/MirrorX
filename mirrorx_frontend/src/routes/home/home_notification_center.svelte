<script context="module" lang="ts">
	import { emit } from '@tauri-apps/api/event';

	export interface HomeNotificationEvent {
		level: 'info' | 'success' | 'warning' | 'error';
		title: string;
		message: string;
	}

	export function emitHomeNotification(notification: HomeNotificationEvent): Promise<void> {
		return emit('home_notification', notification);
	}
</script>

<script lang="ts">
	import { faXmark } from '@fortawesome/free-solid-svg-icons';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { v4 as uuidv4 } from 'uuid';

	let notifications: Array<{ level_color: string; title: string; message: string; id: string }> = [];
	let unlisten_fn: UnlistenFn | null = null;
	$: notifications_reverse = notifications.reverse().slice(0, 2);

	onMount(async () => {
		unlisten_fn = await listen<string>('home_notification', (event) => {
			let payload: HomeNotificationEvent = JSON.parse(event.payload);

			let level_color: string = '';
			switch (payload.level) {
				case 'info':
					level_color = 'bg-info';
				case 'success':
					level_color = 'bg-success';
				case 'warning':
					level_color = 'bg-warning';
				case 'error':
					level_color = 'bg-error';
			}

			notifications.push({
				level_color: level_color,
				title: payload.title,
				message: payload.message,
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
	<div class="toast toast-top toast-center w-full {notifications.length > 0 ? 'z-50' : 'z-0'}">
		<div class="stack">
			{#each notifications_reverse as notification}
				<div
					class="card bg-info text-primary-content card-compact w-full min-w-full shadow-md {notification.level_color}"
				>
					<div class="card-body">
						<h2 class="card-title flex flex-row">
							<div class="flex-1">{notification.title}</div>
							<div class="card-actions">
								<button
									class="btn btn-square btn-sm"
									on:click={() => {
										let index = notifications.findIndex((x) => x.id == notification.id);
										if (index != -1) {
											notifications.splice(index, 1);
											notifications = notifications;
										}
									}}
								>
									<Fa icon={faXmark} />
								</button>
							</div>
						</h2>

						<p>{notification.message}</p>
					</div>
				</div>
			{/each}
		</div>
	</div>
</slot>
