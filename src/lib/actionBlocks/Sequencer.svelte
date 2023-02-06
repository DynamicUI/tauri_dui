<script lang="ts">
	/*
	 * The sequencer represent an array of actions that will be executed in order.
	 * It can be used as lambda function, function declaration, main function, etc.
	 */
	import AddButtonGroup from '../components/AddButtonGroup.svelte';
	import { fade } from 'svelte/transition';
	import { flip } from 'svelte/animate';
	import VariableDeclaration from '$lib/actionBlocks/VariableDeclaration.svelte';
	import { FunctionCall } from '$lib/actionBlocks';
	import { DelButton } from '$lib/components';

	export let sequence: any; // From rust

	function delBlock(id: String) {
		// TODO backend
		sequence.actions = sequence.actions.filter((actionBlock: any) => actionBlock.id != id);
	}

	let last_id = 5;

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
		const { data } = actionBlock;
		if (data.VariableDeclaration) {
			return 'bg-success';
		} else if (data.FunctionCall) {
			return 'bg-warning';
		} else if (data.FunctionDeclaration != undefined) {
			return 'bg-white';
		}
	}
</script>

<div class="items-center justify-center rounded-2xl bg-neutral">
	<div class="flex flex-col items-center overflow-scroll p-16" style:max-height="80vh">
		{#each sequence.actions as actionBlock (actionBlock.id)}
			<div animate:flip={{ duration: 150 }} transition:fade={{ duration: 150 }}>
				<div class="mb-10 flex items-center">
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
						<div class="mx-5 flex flex-row items-center justify-center rounded-3xl bg-gray-800" />
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
						<DelButton id={actionBlock.id} onClick={() => delBlock(actionBlock.id)} />
					{/if}
				</div>
			</div>
		{/each}
	</div>
	<AddButtonGroup {addVariableDecl} {addFunctionCall} />
</div>
