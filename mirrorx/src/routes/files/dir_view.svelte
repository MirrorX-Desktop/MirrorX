<script lang="ts">
	import type { Directory } from '$lib/components/types';
	import { faSpinner } from '@fortawesome/free-solid-svg-icons';
	import moment from 'moment';
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
		const regexp = /^[A-Za-z]:\\$/;
		if (regexp.test(path)) {
			return path;
		}

		let slashPosition = path.lastIndexOf('/');
		if (slashPosition == -1) {
			slashPosition = path.lastIndexOf('\\');
		}

		return path.slice(slashPosition + 1);
	};

	const get_extname = (path: string): string => {
		const baseName = get_basename(path);
		const parts = baseName.split('.');

		switch (parts[parts.length - 1]) {
			case 'ext':
				return 'Application';
			case 'zip':
			case 'rar':
			case '7z':
				return 'Compress Archive';
			default:
				return 'File';
		}
	};

	const get_filesize = (size: number): string => {
		var num = 1024.0; //byte

		if (size < num) return size + 'B';
		if (size < Math.pow(num, 2)) return (size / num).toFixed(2) + 'K'; //kb
		if (size < Math.pow(num, 3)) return (size / Math.pow(num, 2)).toFixed(2) + 'M'; //M
		if (size < Math.pow(num, 4)) return (size / Math.pow(num, 3)).toFixed(2) + 'G'; //G
		return (size / Math.pow(num, 4)).toFixed(2) + 'T'; //T
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
								<p>{moment.unix(dir.modified_time).format('YYYY-MM-DD')}</p>
								<p>{moment.unix(dir.modified_time).format('hh:mm')}</p>
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
											{console.log(file.icon)}
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
								<p>{moment.unix(file.modified_time).format('YYYY-MM-DD')}</p>
								<p>{moment.unix(file.modified_time).format('hh:mm')}</p>
							{/if}
						</td>
						<!--Size-->
						<td>{get_filesize(file.size)}</td>
						<!--Type-->
						<td>{get_extname(file.path)}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
