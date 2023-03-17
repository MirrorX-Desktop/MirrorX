<script lang="ts">
	export let path: string;
	$: path_parts = calc_path_parts(path);

	let slider: HTMLElement;
	let isDown = false;
	let startX = 0;
	let scrollLeft = 0;
	let active: boolean = false;
	let timeoutHandler: NodeJS.Timeout | null;

	const calc_path_parts = (path: string): Array<string> => {
		let parts: Array<string> = [];
		let end = path.length;

		for (let i = path.length; i >= 0; i--) {
			if (path[i] == '/' || path[i] == '\\') {
				parts.push(path.slice(i + 1, end));
				end = i;
				i -= 1;
			}

			if (i <= 0 && end >= 0) {
				let root = path.slice(0, end + 1);
				if (root == '/' || root == '\\') {
					parts.push('Root');
				} else {
					parts.push(root);
				}
			}
		}

		parts = parts.reverse();

		update_breadcrumbs();

		return parts;
	};

	const update_breadcrumbs = () => {
		if (timeoutHandler) {
			clearTimeout(timeoutHandler);
			timeoutHandler = null;
		}

		timeoutHandler = setTimeout(() => {
			if (path_parts && slider && !isDown) {
				slider.scrollLeft = slider.scrollWidth;
			}
		}, 100);
	};

	const mouse_down = (e: MouseEvent) => {
		isDown = true;
		active = true;
		startX = e.pageX - slider.offsetLeft;
		scrollLeft = slider.scrollLeft;
	};

	const mouse_leave = (e: MouseEvent) => {
		isDown = false;
		active = false;
	};

	const mouse_up = (e: MouseEvent) => {
		isDown = false;
		active = false;
	};

	const mouse_move = (e: MouseEvent) => {
		if (!isDown) {
			return;
		}
		e.preventDefault();
		const x = e.pageX - slider.offsetLeft;
		const walk = (x - startX) * 2;
		slider.scrollLeft = scrollLeft - walk;
	};
</script>

<div class="breadcrumbs mx-2 max-w-fit overflow-hidden text-xs">
	<ul
		bind:this={slider}
		class=" cursor-pointer select-none overflow-hidden whitespace-nowrap {active ? 'active' : ''}"
		on:mousedown={mouse_down}
		on:mouseleave={mouse_leave}
		on:mouseup={mouse_up}
		on:mousemove={mouse_move}
	>
		{#each path_parts as path_part}
			<li>{path_part}</li>
		{/each}
	</ul>
</div>

<style>
	.active {
		@apply scale-100 cursor-grabbing;
	}
</style>
