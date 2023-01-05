<script lang="ts">
	import { invoke_file_manager_visit_local, invoke_file_manager_visit_remote } from '$lib/components/command';
	import type { Directory, Entry } from '$lib/components/types';
	import {
		faHome,
		faArrowLeft,
		faArrowRight,
		faArrowUp,
		faArrowDownAZ,
		faChevronDown,
		faSpinner
	} from '@fortawesome/free-solid-svg-icons';
	import moment from 'moment';
	import Fa from 'svelte-fa';
	import { onMount } from 'svelte';
	import { path } from '@tauri-apps/api';
	import Bread from './bread.svelte';

	export let remoteDeviceID: string | null;
	$: isLocal = remoteDeviceID == null;

	let directory: Directory | null = null;
	let path_input: HTMLInputElement;
	let has_parent: boolean = false;
	let has_back: boolean = false;
	let has_forward: boolean = false;
	let visit_record: Array<string | null> = [];
	let visit_pos: number = 0;

	onMount(async () => {
		await goto(null);
	});

	const sort_entries = (entries: Array<Entry>): Array<Entry> => {
		let dirs = entries.filter((v) => v.is_dir).sort((a, b) => a.path.localeCompare(b.path));
		let files = entries.filter((v) => !v.is_dir).sort((a, b) => a.path.localeCompare(b.path));
		return dirs.concat(files);
	};

	const get_basename = (path: string): string => {
		const regexp = /^[A-Za-z]:\\$/;
		if (regexp.test(path)) {
			return path;
		}

		if (path == '/' || path == '\\') {
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

	const goto_root = async () => {
		if (directory && (directory.path == '/' || directory.path == '\\')) {
			return;
		}

		await goto(null);
	};

	const update_has_parent = () => {
		if (directory) {
			has_parent = directory.path != '/' && directory.path != '\\';
			return;
		}
		has_parent = false;
	};

	const update_has_back = () => {
		has_back = visit_pos > 1;
	};

	const update_has_forward = () => {
		has_forward = visit_pos < visit_record.length;
	};

	const goto_parent = async () => {
		if (has_parent && directory) {
			let currentDir = get_basename(directory.path);
			let part = directory.path.slice(0, directory.path.length - currentDir.length - 1);
			if (part.length == 0) {
				await goto(null);
			} else {
				await goto(part);
			}
		}
	};

	const goto_back = async () => {
		if (visit_pos > 1) {
			try {
				let path = visit_record[visit_pos - 1 - 1];
				if (path == '/' || path == '\\') {
					path = null;
				}

				let new_dir = await visit_dir(path);
				visit_pos--;
				directory = new_dir;

				update_toolbar();
			} catch (err: any) {
				console.log(err);
			}
		}
	};

	const goto_forward = async () => {
		if (visit_pos < visit_record.length) {
			try {
				let path = visit_record[visit_pos - 1 + 1];
				if (path == '/' || path == '\\') {
					path = null;
				}

				let new_dir = await visit_dir(path);
				visit_pos++;
				directory = new_dir;

				update_toolbar();
			} catch (err: any) {
				console.log(err);
			}
		}
	};

	const goto = async (path: string | null) => {
		try {
			let new_dir = await visit_dir(path);
			visit_record = visit_record.slice(0, visit_pos); // discard old forward records
			visit_record.push(new_dir.path);
			visit_pos++;
			directory = new_dir;

			update_toolbar();
		} catch (err: any) {
			console.log(err);
		}
	};

	const update_toolbar = () => {
		if (path_input) {
			path_input.scrollLeft = path_input.scrollWidth;
		}

		update_has_back();
		update_has_forward();
		update_has_parent();
	};

	const visit_dir = async (path: string | null): Promise<Directory> => {
		let dir: Directory;

		if (isLocal) {
			dir = await invoke_file_manager_visit_local(path);
		} else {
			dir = await invoke_file_manager_visit_remote(remoteDeviceID!, path);
		}

		dir.entries = sort_entries(dir.entries);
		return dir;
	};
</script>

<div class="flex h-full w-full flex-col items-center justify-center">
	{#if directory}
		<!--ToolBar-->
		<div class="flex-0 flex w-full flex-row gap-2">
			<div class="btn-group flex-0">
				<div class="tooltip tooltip-bottom z-50" data-tip="Root Directory">
					<button class="btn btn-sm rounded-tr-none rounded-br-none" on:click={goto_root}>
						<Fa icon={faHome} />
					</button>
				</div>

				<div class="tooltip tooltip-bottom z-50" data-tip="hello">
					<button class="btn btn-sm rounded-none {has_back ? '' : 'btn-disabled'}" on:click={goto_back}>
						<Fa icon={faArrowLeft} />
					</button>
				</div>

				<div class="tooltip tooltip-bottom z-50" data-tip="hello">
					<button class="btn btn-sm rounded-none {has_forward ? '' : 'btn-disabled'}" on:click={goto_forward}>
						<Fa icon={faArrowRight} />
					</button>
				</div>

				<div class="tooltip tooltip-bottom z-50" data-tip="hello">
					<button
						class="btn btn-sm rounded-tl-none rounded-bl-none {has_parent ? '' : 'btn-disabled'}"
						on:click={goto_parent}
					>
						<Fa icon={faArrowUp} />
					</button>
				</div>
			</div>

			<div class="form-control flex-1">
				<div class="input-group">
					<input
						bind:this={path_input}
						type="text"
						class="input input-sm input-bordered ring-info focus:border-info w-full text-center focus:outline-none focus:ring"
						placeholder={get_basename(directory.path)}
					/>
					<button class="btn btn-sm">
						<Fa icon={faChevronDown} />
					</button>
				</div>
			</div>

			<!-- <div class="flex-0">
				<button class="btn btn-sm"><Fa icon={faArrowDownAZ} /></button>
			</div> -->
		</div>

		<div class="flex-0 max-w-full">
			<Bread path={directory.path} />
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
					{#each directory.entries as entry}
						<tr
							class="hover"
							on:click={() => {
								if (entry.is_dir) {
									goto(entry.path);
								}
							}}
						>
							<!--Icon-->
							<td>
								<div class="flex h-full flex-row items-center justify-center">
									{#if entry.icon}
										<img style="width: 32px; height:32px" src={'data:image/png;base64,' + entry.icon} alt="File Icon" />
									{/if}
								</div>
							</td>
							<!--Name-->
							<td>
								<div class="name-content">{get_basename(entry.path)}</div>
								{#if !entry.is_dir}
									<div class="text-xs opacity-50">{get_extname(entry.path)}</div>
								{/if}
							</td>
							<!--Modified Date-->
							<td>
								{#if entry.modified_time != 0}
									<p class="text-right text-sm">{moment.unix(entry.modified_time).format('YYYY-MM-DD')}</p>
									<p class="text-right text-sm">{moment.unix(entry.modified_time).format('hh:mm')}</p>
								{/if}
							</td>
							<!--Size-->
							<td class="text-center text-sm">
								{#if !entry.is_dir}
									{get_filesize(entry.size)}
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{:else}
		<Fa icon={faSpinner} spin />
	{/if}
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
