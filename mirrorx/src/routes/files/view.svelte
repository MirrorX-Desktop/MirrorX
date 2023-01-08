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
	import { onMount, onDestroy } from 'svelte';
	import Bread from './bread.svelte';
	import { current_remote_directory } from '$lib/components/stores';
	import type { Unsubscriber } from 'svelte/store';
	import { emit } from '@tauri-apps/api/event';
	import { save } from '@tauri-apps/api/dialog';

	export let remoteDeviceID: string | null;
	$: isLocal = remoteDeviceID == null;

	let view: HTMLDivElement;

	let directory: Directory | null = null;
	let remote_directory: Directory | null = null;
	let path_input: HTMLInputElement;
	let path_input_value: string;
	let path_input_record: Array<string> = [];
	$: path_input_record_display = path_input_record.slice().reverse();

	let has_parent: boolean = false;
	let has_back: boolean = false;
	let has_forward: boolean = false;
	let visit_record: Array<string | null> = [];
	let visit_pos: number = 0;

	let contextMenu: HTMLDivElement;
	let showMenu: boolean = false;
	let contextMenuRelatedEntry: Entry | null = null;

	let current_remote_directory_unsubscriber: Unsubscriber | null = null;

	onMount(async () => {
		if (isLocal) {
			current_remote_directory_unsubscriber = current_remote_directory.subscribe((dir) => {
				remote_directory = dir;
			});
		}

		await goto(null);
	});

	onDestroy(() => {
		if (current_remote_directory_unsubscriber) {
			current_remote_directory_unsubscriber();
		}
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

	const input_goto = async (event: KeyboardEvent) => {
		if (path_input_value && path_input_value.length > 0 && event.code == 'Enter') {
			let goto_path: string | null = path_input_value;
			if (path_input_value == '/' || path_input_value == '\\') {
				goto_path = null;
			}

			try {
				let new_dir = await visit_dir(goto_path);
				visit_record = visit_record.slice(0, visit_pos); // discard old forward records
				visit_record.push(new_dir.path);
				visit_pos++;
				directory = new_dir;

				update_toolbar();

				if (!path_input_record.includes(new_dir.path)) {
					if (path_input_record.length == 10) {
						path_input_record.shift();
					}

					path_input_record.push(new_dir.path);
					path_input_record = path_input_record;
					console.log(path_input_record);
				}
			} catch (err: any) {
				console.log(err);
			}
		}
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

	const showFileMenu = (event: MouseEvent, entry: Entry) => {
		event.preventDefault();

		if (entry.is_dir) {
			return;
		}

		console.log('click menu at :' + entry.path);

		showMenu = true;

		// make sure context menu will not overflow the view

		let left = event.clientX;
		let top = event.clientY;

		if (event.clientX + contextMenu.clientWidth > view.offsetLeft + view.clientWidth) {
			left = event.clientX - contextMenu.clientWidth;
		}

		if (event.clientY + contextMenu.clientHeight > view.offsetTop + view.clientHeight) {
			top = event.clientY - contextMenu.clientHeight;
		}

		contextMenu.style.left = left + 'px';
		contextMenu.style.top = top + 'px';
	};

	const dismissFileMenu = () => {
		showMenu = false;
		contextMenuRelatedEntry = null;
	};

	const checkShouldDismissFileMenu = (event: MouseEvent) => {
		if (showMenu && contextMenu) {
			let menuRect = contextMenu.getBoundingClientRect();

			if (
				!(
					event.clientX >= menuRect.left &&
					event.clientX <= menuRect.left + menuRect.width &&
					event.clientY >= menuRect.top &&
					event.clientY <= menuRect.top + menuRect.height
				)
			) {
				dismissFileMenu();
			}
		}
	};

	const send_to = async () => {
		const entry = contextMenuRelatedEntry;
		dismissFileMenu();

		if (!entry) {
			return;
		}

		if (isLocal) {
			// send to remote

			if (!remote_directory) {
				return;
			}

			if (remote_directory.path == '/' || remote_directory.path == '\\') {
				// todo: notify send data to root dir is disallowed
				return;
			}

			await emit('send_file_to_remote', { entry, path: remote_directory.path });
		} else {
			// download to local

			let basename = get_basename(entry.path);
			let nameAndExtension = basename.split('.');
			let name = nameAndExtension[0];
			let extensions: Array<string> = [];
			if (nameAndExtension.length == 2) {
				extensions = [nameAndExtension[1]];
			}

			const filePath = await save({
				filters: [
					{
						name,
						extensions
					}
				]
			});

			if (!filePath) {
				return;
			}

			await emit('download_file_to_local', { localPath: filePath, remotePath: entry.path });
		}
	};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div bind:this={view} class="h-full w-full" on:click={checkShouldDismissFileMenu}>
	<div bind:this={contextMenu} class="absolute z-50 {showMenu ? 'visible' : 'invisible'}">
		<ul class="menu bg-base-100 rounded-box w-56 p-2 shadow">
			{#if isLocal}
				<li><button on:click={send_to}>Send to Remote</button></li>
			{:else}
				<li><button on:click={send_to}>Download to Local</button></li>
			{/if}
		</ul>
	</div>

	<div class="flex h-full w-full flex-col items-center justify-center">
		{#if directory}
			<!--ToolBar-->
			<div class="flex-0 flex w-full flex-row gap-2">
				<div class="btn-group flex-0">
					<button class="btn btn-sm" on:click={goto_root}>
						<Fa icon={faHome} />
					</button>

					<button class="btn btn-sm {has_back ? '' : 'btn-disabled'}" on:click={goto_back}>
						<Fa icon={faArrowLeft} />
					</button>

					<button class="btn btn-sm {has_forward ? '' : 'btn-disabled'}" on:click={goto_forward}>
						<Fa icon={faArrowRight} />
					</button>

					<button class="btn btn-sm {has_parent ? '' : 'btn-disabled'}" on:click={goto_parent}>
						<Fa icon={faArrowUp} />
					</button>
				</div>

				<div class="form-control flex-1">
					<div class="input-group">
						<input
							bind:this={path_input}
							type="text"
							class="input input-sm input-bordered ring-info focus:border-info z-10 w-full text-center focus:outline-none focus:ring"
							placeholder={get_basename(directory.path)}
							bind:value={path_input_value}
							on:keyup={input_goto}
						/>
						<!-- <button class="btn btn-sm">
						
					</button> -->
						<div class="dropdown dropdown-bottom dropdown-end">
							<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
							<!-- svelte-ignore a11y-label-has-associated-control -->
							<label tabindex="0" class="btn btn-sm z-0 rounded-tl-none rounded-bl-none">
								<Fa icon={faChevronDown} />
							</label>
							<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
							<ul
								tabindex="0"
								class="dropdown-content menu bg-base-300 mt-1 overflow-hidden overflow-ellipsis rounded-lg p-2 shadow"
								style="min-width: 160px; max-width: calc(100vw / 2 * 0.8)"
							>
								<li class="menu-title">
									<span>Recent 10 records</span>
								</li>
								{#if path_input_record_display.length > 0}
									{#each path_input_record_display as record}
										<li class="w-full">
											<button class="inline w-full overflow-hidden overflow-ellipsis whitespace-nowrap text-left">
												{record}
											</button>
										</li>
									{/each}
								{:else}
									<li class="text-base-content text-center text-sm text-opacity-60">Empty</li>
								{/if}
							</ul>
						</div>
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
									if (showMenu) {
										return;
									}

									if (entry.is_dir) {
										goto(entry.path);
									}
								}}
								on:contextmenu={(event) => showFileMenu(event, entry)}
							>
								<!--Icon-->
								<td>
									<div class="flex h-full flex-row items-center justify-center">
										{#if entry.icon}
											<img
												style="width: 32px; height:32px"
												src={'data:image/png;base64,' + entry.icon}
												alt="File Icon"
											/>
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