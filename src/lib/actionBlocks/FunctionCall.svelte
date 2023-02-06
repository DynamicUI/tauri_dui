<script lang="ts">
	import { Input } from '$lib/class/input';
	import { Vec2 } from '$lib/class/vec2';
	import { onMount } from 'svelte';
	import { TextInput } from '$lib/dataBlocks';
	import { variableDeclarationSize } from './defaultValues';
	import MultiInput from '$lib/dataBlocks/MultiInput.svelte';

	export let function_: any;
	export let actionId: number;

	let inputName: Input = {
		target: undefined,
		isWriteMode: false,
		value: '',
		isFocus: false
	};

	onMount(() => {
		console.log('function_ :>> ', function_);
		//inputName.isWriteMode = true;
		//$isDialogInputOpen = true;
		for (let i = 0; i < function_.args.length; i++) {
			function_.args[i].input = {
				target: undefined,
				isWriteMode: false,
				value: '',
				isFocus: false
			};
		}
		console.log('function_ :>> ', function_);
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

	$: cannotDisplayArgs = function_.args.length === 0 || function_.args[0].input === undefined;
</script>

<!--style:min-width="{variableDeclarationSize.x}px"-->
<!--style:min-height="{variableDeclarationSize.y}px"-->
<div
	class="mx-5 flex flex-row items-center justify-center rounded-3xl border-2 border-white bg-base-100 p-2"
>
	<TextInput
		bind:input={inputName}
		bind:value={function_.name}
		giveInput={validateNameInput}
		isInputValid={isVarNameValid}
	/>

	{#if cannotDisplayArgs}
		<span class="mx-2">=</span>
	{:else}
		{#each function_.args as arg}
			<MultiInput
				{actionId}
				value={arg}
				isInputValid={() => true}
				giveInput={() => true}
				input={arg.input}
			/>
		{/each}
	{/if}
</div>
