<script lang="ts">
	import { Input } from '$lib/class/input';
	import { Vec2 } from '$lib/class/vec2';
	import { onMount } from 'svelte';
	import { TextInput } from '$lib/dataBlocks';
	import { variableDeclarationSize } from './defaultValues';

	export let function_: any;
	export let actionId: number;

	let input_size: Vec2 = new Vec2(
		variableDeclarationSize.x / 2 - 10,
		variableDeclarationSize.y - 20
	);

	let inputName: Input = {
		target: undefined,
		isWriteMode: false,
		value: '',
		isFocus: false
	};

	onMount(() => {
		//inputName.isWriteMode = true;
		//$isDialogInputOpen = true;
	});

	function validateNameInput(value: any) {
		if (doesFnExist(value)) return;
		// TODO change
	}

	function isVarNameValid(name: string): boolean {
		return !doesFnExist(name);
	}

	function doesFnExist(name: string): boolean {
		//return builtins.has(name);
		return false;
	}

	function validateArgInput() {
		return true;
	}

	function isArgInputValide(value: string): boolean {
		return true;
	}
</script>

<!--style:min-width="{variableDeclarationSize.x}px"-->
<!--style:min-height="{variableDeclarationSize.y}px"-->
<div
	class="mx-5 flex flex-row items-center justify-center rounded-3xl bg-base-100"
>
	<TextInput
		bind:input={inputName}
		bind:input_size
		bind:value={function_.name}
		giveInput={validateNameInput}
		isInputValid={isVarNameValid}
	/>

	{#each function_.args as arg}
		{arg}
		{arg.Variable}
		<!-- 
		<MulitInput
			bind:input={arg.input_value}
			bind:input_size
			bind:value={arg.variable.input}
			{actionId}
			giveInput={validateArgInput}
			isInputValid={isArgInputValide}
		/>

			OLD-->
	{/each}
</div>
