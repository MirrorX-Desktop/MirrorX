<script lang="ts">
	import Appearance from './appearance.svelte';
</script>

<slot>
	<div class="flex h-full w-full flex-col overflow-y-auto py-2 px-2">
		<Appearance />
	</div>
</slot>
