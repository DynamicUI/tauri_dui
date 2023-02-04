<script lang="ts">
	/*
	 * The sequencer represent an array of actions that will be executed in order.
	 * It can be used as lambda function, function declaration, main function, etc.
	 */
	import { register, unregisterAll } from '@tauri-apps/api/globalShortcut';
	import AddButtonGroup from './AddButtonGroup.svelte';
	import { onDestroy, onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { flip } from 'svelte/animate';
	import Alert from './Alert.svelte';
	import { variableDeclarationSize } from '$lib/actionBlocks/defaultValues';
	import VariableDeclaration from '$lib/actionBlocks/VariableDeclaration.svelte';
	import { FunctionCall } from '$lib/actionBlocks';

	export let sequence: any; // From rust

	onMount(() => {
		sequence.actions = [...sequence.actions, { id: 100, data: null }];
	});

	let last_id = 5;
	function delBlock(id: String) {
		sequence.actions = sequence.actions.filter((actionBlock: any) => actionBlock.id != id);
	}

	function addFunctionCall() {
		console.log('add f call');
		// TODO backend
		sequence.actions = [
			...sequence.actions,
			{ id: last_id, data: { FunctionCall: { name: 'test', args: [] } } }
		];
		last_id += 1;
	}

	function addVariableDecl() {
		console.log('add v call');
		// TODO backend
		sequence.actions = [
			...sequence.actions,
			{ id: last_id, data: { VariableDeclaration: { name: 'test' } } }
		];
		last_id += 1;
	}

	function badgeColor(actionBlock: any) {
		if (actionBlock.data.VariableDeclaration) {
			return 'bg-success';
		} else if (actionBlock.data.FunctionCall) {
			return 'bg-warning';
		} else if (actionBlock.data.FunctionDeclaration) {
			return 'bg-white';
		}
	}
</script>

id: {sequence.id} context: {sequence.context}
<div class="bg-neutral items-center  rounded-2xl justify-center">
	<div style="width: {variableDeclarationSize.x * 1.9}px;" />
	<div class="flex  flex-col p-16 overflow-scroll h-[80vh] items-center">
		{#each sequence.actions as actionBlock (actionBlock.id)}
			<div animate:flip={{ duration: 150 }} transition:fade={{ duration: 150 }}>
				<div class="flex items-center mb-10">
					{#if actionBlock.data != null}
						<span
							class="badge {badgeColor(actionBlock)}"
							draggable="true"
							on:dragstart={(event) => {
								let send_data = {
									actionId: actionBlock.id,
									data: actionBlock.data
								};
								event?.dataTransfer?.setData('dragDatas', JSON.stringify(send_data));
							}}
						/>
					{/if}

					{#if actionBlock.data == null}
						<div
							style:width="{variableDeclarationSize.x}px"
							style:height="{variableDeclarationSize.y}px"
							class="bg-gray-800 flex flex-row rounded-3xl mx-5 justify-center items-center"
						/>
					{:else if actionBlock.data.VariableDeclaration}
						<VariableDeclaration
							actionId={actionBlock.id}
							bind:variable={actionBlock.data.VariableDeclaration}
						/>
					{:else if actionBlock.data.FunctionCall}
						<FunctionCall
							actionId={actionBlock.id}
							bind:function_={actionBlock.data.FunctionCall}
						/>
					{/if}

					{#if actionBlock.data != null}
						<button
							class="btn btn-circle bg-red-400 hover:bg-red-700"
							on:click={() => {
								delBlock(actionBlock.id);
							}}
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-6 w-6 text-black"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M6 18L18 6M6 6l12 12"
								/></svg
							>
						</button>
					{/if}
				</div>
			</div>
		{/each}
	</div>
	<AddButtonGroup {addVariableDecl} {addFunctionCall} />
</div>

<Alert enabled={false} msg={"You can't save, idiot..."} />
