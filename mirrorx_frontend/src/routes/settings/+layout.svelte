<script lang="ts">
	import SettingsNotificationCenter from './settings_notification_center.svelte';
	import { page } from '$app/stores';
</script>

<div class="flex h-full w-full flex-row ">
	<div class="h-full border-r">
		<ul class="menu bg-base-100 w-56">
			<li>
				<a href="/settings/domain" class={$page.url.pathname == '/settings/domain' ? 'active' : ''}>Domain</a>
			</li>
			<li>
				<a href="/settings/about" class={$page.url.pathname == '/settings/about' ? 'active' : ''}>About</a>
			</li>
		</ul>
	</div>

	<div class="h-full flex-1">
		<slot />
	</div>
</div>

<SettingsNotificationCenter />
