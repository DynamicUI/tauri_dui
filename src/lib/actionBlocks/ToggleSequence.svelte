<script lang="ts">
	/*
	 * The toggle sequencer a sequence in a lambda context.
	 */
	import AddButtonGroup from '../components/AddButtonGroup.svelte';
	import FunctionCall from '$lib/actionBlocks/FunctionCall.svelte';
	import VariableDeclaration from '$lib/actionBlocks/VariableDeclaration.svelte';
	import DelButton from '$lib/components/DelButton.svelte';

	export let sequence: any; // from frontend

	let last_id = 10;

	function addVariableDecl() {
		console.log('add v call');
		// TODO backend
		sequence.actions = [
			...sequence.actions,
			{ id: last_id, data: { VariableDeclaration: { name: 'test' } } }
		];
		last_id += 1;
	}

	function delBlock(id: String) {
		// TODO backend
		sequence.actions = sequence.actions.filter((actionBlock: any) => actionBlock.id != id);
	}
</script>

<div
	class="items-center justify-center rounded-2xl bg-neutral"
	on:dblclick={() => {
		// TODO goto a window with the clicked sequence
		//sequence.actions = [...sequence.actions, { id: 100, data: null }];
	}}
>
	<div class="flex flex-col items-center">
		{#each sequence.actions as actionBlock (actionBlock.id)}
			<div class="flex items-center">
				{#if actionBlock.data != null}
					<DelButton id={actionBlock.id} onClick={() => delBlock(actionBlock.id)} />
				{/if}

				{#if actionBlock.data == null}
					yes
				{:else if actionBlock.data.VariableDeclaration}
					<VariableDeclaration
						actionId={actionBlock.id}
						bind:variable={actionBlock.data.VariableDeclaration}
					/>
				{:else if actionBlock.data.FunctionCall}
					<FunctionCall actionId={actionBlock.id} bind:function_={actionBlock.data.FunctionCall} />
				{/if}
			</div>
		{/each}
		<AddButtonGroup {addVariableDecl} addFunctionCall={null} />
	</div>
</div>
