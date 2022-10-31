<script lang="ts">
	import { faXmark } from '@fortawesome/free-solid-svg-icons';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';
	import Fa from 'svelte-fa';
	import { v4 as uuidv4 } from 'uuid';
	import type { NotificationEvent } from '../event_types';

	var notifications: Array<{ level_color: string; title: string; message: string; id: string }> = [];

	$: notifications_reverse = notifications.reverse().slice(0, 2);

	onMount(async () => {
		await listen<string>('notification', (event) => {
			let payload: NotificationEvent = JSON.parse(event.payload);

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
</script>

<slot>
	<div class="toast toast-top toast-center">
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
