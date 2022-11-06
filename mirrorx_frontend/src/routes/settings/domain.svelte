<script lang="ts">
	import { faPenToSquare, faPlus, faThumbTack, faTrash, faTrashCan, faXmark } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
</script>

<slot>
	<div class="mx-2 h-full">
		<div class="flex flex-row items-center justify-between py-3">
			<span class="text-2xl">Current: MirrorX.clould</span>
			<button class="btn btn-xs"><Fa icon={faPlus} /></button>
		</div>
		<hr />

		<div id="domain-table" class="h-80 w-full overflow-y-auto overflow-x-hidden">
			<table class="table w-full">
				<tbody>
					<tr>
						<th>1</th>
						<td class="w-full">
							<p class="text-lg">www.Abcdowkaod-sa.com</p>
							<p>15-5555-6666</p>
							<p>备注：</p>
						</td>
						<td class="text-right">
							<div class="btn-group">
								<!-- <button class="btn btn-xs"><Fa icon={faThumbTack} /></button> -->
								<button class="btn btn-xs"><Fa icon={faPenToSquare} /></button>
								<button class="btn btn-xs"><Fa icon={faTrashCan} /></button>
							</div>
						</td>
					</tr>
					<tr>
						<th>2</th>
						<td class="w-full">
							<p class="text-lg">www.Abcdowkaod-sa.com</p>
							<p>15-5555-6666</p>
						</td>
						<td>
							<div class="btn-group">
								<button class="btn btn-xs"><Fa icon={faThumbTack} /></button>
								<button class="btn btn-xs"><Fa icon={faPenToSquare} /></button>
								<button class="btn btn-xs"><Fa icon={faTrashCan} /></button>
							</div>
						</td>
					</tr>
				</tbody>
			</table>
		</div>
		<hr />
	</div>
</slot>

<style>
	#domain-table::-webkit-scrollbar {
		@apply absolute right-0 w-2 rounded;
	}

	#domain-table::-webkit-scrollbar-thumb {
		@apply absolute right-0 rounded bg-gray-600;
	}

	#domain-table::-webkit-scrollbar-track {
		@apply absolute right-0 w-2 rounded bg-gray-50;
	}
</style>
