<script lang="ts">
	import type { Directory } from '$lib/components/types';
	import { faSpinner } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	export let directory: Directory;
	export let clickItem: (path: string) => void;

	const convert_png = async (bytes: Uint8Array): Promise<string | ArrayBuffer | null> => {
		let blob = new Blob([bytes], { type: 'image/png' });
		console.log('length ' + bytes.byteLength + ';' + blob.length);

		return new Promise((resolve, _) => {
			const reader = new FileReader();
			reader.onloadend = () => resolve(reader.result);
			reader.readAsDataURL(blob);
		});
	};

	const get_basename = (path: string): string => {
		let slashPosition = path.lastIndexOf('/');
		if (slashPosition == -1) {
			slashPosition = path.lastIndexOf('\\');
		}

		if (slashPosition == -1) {
			slashPosition = 0;
		}
		console.log(slashPosition);
		return path.slice(slashPosition);
	};
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
					<tr class="hover" on:click={() => clickItem(dir.path)}>
						<!--Name-->
						<td>
							<div class="flex items-center space-x-3">
								<div class="avatar">
									<div class="mask mask-squircle flex h-12 w-12 flex-row items-center justify-center">
										{#if dir.icon}
											<img style="width: 32px; height:32px" src={'data:image/png;base64,' + dir.icon} alt="File Icon" />
										{/if}
									</div>
								</div>
								<div>
									<div class="font-bold">{get_basename(dir.path)}</div>
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

				{#each directory.files as file}
					<tr class="hover">
						<!--Name-->
						<td>
							<div class="flex items-center space-x-3">
								<div class="avatar">
									<div class="mask mask-squircle flex h-12 w-12 flex-row items-center justify-center">
										{#if file.icon}
											<img
												style="width: 32px; height:32px"
												src={'data:image/png;base64,' + file.icon}
												alt="File Icon"
											/>
										{/if}
									</div>
								</div>
								<div>
									<div class="font-bold">{get_basename(file.path)}</div>
									<!-- <div class="text-sm opacity-50">United States</div> -->
								</div>
							</div>
						</td>
						<!--Modified Date-->
						<td>
							{#if file.modified_time != 0}
								{file.modified_time}
							{/if}
						</td>
						<!--Size-->
						<td>{file.size}</td>
						<!--Type-->
						<td> Directory </td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
