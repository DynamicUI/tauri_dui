<script lang="ts">
	import type { Box } from '$lib/class/box';
	import { Input } from '$lib/class/input';
	import { Vec2 } from '$lib/class/vec2';
	import { onMount } from 'svelte';
	import { Function_ } from '$lib/class/function';
	//import TextInput from './box/TextInput.svelte';
	import { isDialogInputOpen } from '$lib/store';
	import { MulitInput, TextInput } from '$lib/dataBlocks';
	import { variableDeclarationSize } from './defaultValues';
	//import { builtins, getParamNames } from '$lib/builtins';
	//import MulitInput from './box/MulitInput.svelte';

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

<div
	style:width="{variableDeclarationSize.x}px"
	style:height="{variableDeclarationSize.y}px"
	class="bg-base-100 flex flex-row rounded-3xl mx-5 justify-center items-center"
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