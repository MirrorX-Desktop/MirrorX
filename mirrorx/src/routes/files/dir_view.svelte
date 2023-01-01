<script lang="ts">
	import type { Directory } from '$lib/components/types';

	export let directory: Directory;
	export let clickItem: (path: string) => void;
</script>

<div class="h-full w-full">
	<div class="w-full overflow-x-auto">
		<table class="table-compact table w-full">
			<thead>
				<tr>
					<th>Name</th>
					<th>Modified Date</th>
					<th>Size</th>
					<th>Type</th>
				</tr>
			</thead>
			<tbody>
				{#each directory.sub_dirs as dir}
					<tr class="hover">
						<!--Name-->
						<td>
							<div class="flex items-center space-x-3">
								<div class="avatar">
									<div class="mask mask-squircle h-12 w-12">
										<img src="/tailwind-css-component-profile-2@56w.png" alt="Avatar Tailwind CSS Component" />
									</div>
								</div>
								<div>
									<div class="font-bold">{dir.path}</div>
									<!-- <div class="text-sm opacity-50">United States</div> -->
								</div>
							</div>
						</td>
						<!--Modified Date-->
						<td>
							{#if dir.modified_time != 0}
								{dir.modified_time}
							{/if}
						</td>
						<!--Size-->
						<td />
						<!--Type-->
						<td> Directory </td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	{#each directory.files as file}
		<div>{file.path}&nbsp;{file.modified_time}&nbsp;{file.size}</div>
	{/each}
</div>
