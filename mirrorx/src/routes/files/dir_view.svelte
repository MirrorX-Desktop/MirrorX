<script lang="ts">
	import type { Directory } from '$lib/components/types';
	import { faHome, faArrowLeft, faArrowRight, faArrowUp } from '@fortawesome/free-solid-svg-icons';
	import moment from 'moment';
	import Fa from 'svelte-fa';

	export let directory: Directory;
	export let clickItem: (path: string | null) => void;

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
			case 'exe':
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

	const goto_home = () => {
		clickItem(null);
	};
</script>

<div class="flex h-full w-full flex-col p-2">
	<!--ToolBar-->
	<div class="flex w-full flex-row">
		<div class="tooltip tooltip-bottom z-50" data-tip="Root Directory">
			<button class="btn btn-sm rounded-tr-none rounded-br-none" on:click={goto_home}><Fa icon={faHome} /></button>
		</div>

		<div class="tooltip tooltip-bottom z-50" data-tip="hello">
			<button class="btn btn-sm rounded-none"><Fa icon={faArrowLeft} /></button>
		</div>

		<div class="tooltip tooltip-bottom z-50" data-tip="hello">
			<button class="btn btn-sm rounded-none"><Fa icon={faArrowRight} /></button>
		</div>

		<div class="tooltip tooltip-bottom z-50" data-tip="hello">
			<button class="btn btn-sm rounded-tl-none rounded-bl-none"><Fa icon={faArrowUp} /></button>
		</div>
	</div>
	<div class="file-view w-full flex-1 overflow-x-auto">
		<table class="w-full table-fixed">
			<thead>
				<tr>
					<th style="width: 48px;" />
					<th class="text-left" style="width: calc(60%-48px);">Name</th>
					<th class="text-right" style="width: 20%;">Modified Date</th>
					<th class="text-center" style="width: 20%;">Size</th>
				</tr>
			</thead>
			<tbody>
				{#each directory.sub_dirs as dir}
					<tr class="hover" on:click={() => clickItem(dir.path)}>
						<!--Icon-->
						<td>
							<div class="flex h-full flex-row items-center justify-center">
								{#if dir.icon}
									<img style="width: 32px; height:32px;" src={'data:image/png;base64,' + dir.icon} alt="File Icon" />
								{/if}
							</div>
						</td>
						<!--Name-->
						<td>
							<div class="name-content">{get_basename(dir.path)}</div>
						</td>
						<!--Modified Date-->
						<td>
							{#if dir.modified_time != 0}
								<p class="text-right text-sm">{moment.unix(dir.modified_time).format('YYYY-MM-DD')}</p>
								<p class="text-right text-sm">{moment.unix(dir.modified_time).format('hh:mm')}</p>
							{/if}
						</td>
						<!--Size-->
						<td />
					</tr>
				{/each}

				{#each directory.files as file}
					<tr class="hover">
						<!--Icon-->
						<td>
							<div class="flex h-full flex-row items-center justify-center">
								{#if file.icon}
									<img style="width: 32px; height:32px" src={'data:image/png;base64,' + file.icon} alt="File Icon" />
								{/if}
							</div>
						</td>
						<!--Name-->
						<td>
							<div class="name-content">{get_basename(file.path)}</div>
							<div class="text-xs opacity-50">{get_extname(file.path)}</div>
						</td>
						<!--Modified Date-->
						<td>
							{#if file.modified_time != 0}
								<p class="text-right text-sm">{moment.unix(file.modified_time).format('YYYY-MM-DD')}</p>
								<p class="text-right text-sm">{moment.unix(file.modified_time).format('hh:mm')}</p>
							{/if}
						</td>
						<!--Size-->
						<td class="text-center text-sm">{get_filesize(file.size)}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>

<style>
	table > thead > tr :where(th) {
		@apply bg-base-300 text-base-content sticky top-0 z-10 text-sm;
	}

	table > tbody :where(td) {
		@apply border-b p-1;
	}

	table > tbody :where(tr):hover {
		@apply bg-base-300;
	}

	.name-content {
		@apply text-sm font-bold;
		display: -webkit-box !important;
		-webkit-box-orient: vertical !important;
		-webkit-line-clamp: 2 !important;
		overflow: hidden !important;
		word-break: break-all !important;
		white-space: normal !important;
	}

	.file-view::-webkit-scrollbar {
		@apply w-1;
	}

	.file-view::-webkit-scrollbar-thumb {
		@apply bg-base-300 rounded-full;
	}

	.file-view::-webkit-scrollbar-track {
		@apply bg-transparent;
	}
</style>
