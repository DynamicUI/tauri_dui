<script lang="ts">
	import { Variable, variables, variable_names } from '$lib/class/variable';
	import { Input, Vec2 } from '$lib/class';
	import { variableDeclarationSize } from './defaultValues';
	import { MultiInput, TextInput } from '$lib/dataBlocks';
	import { invoke } from '@tauri-apps/api';
	import { currentSequenceId } from '$lib/store';
	import { onMount } from 'svelte';

	export let variable: any;
	export let actionId: number;
	onMount(() => {
		console.info('variableDeclaration onMount', variable.name);
	});

	let inputName: Input = {
		target: undefined,
		isWriteMode: false,
		value: '',
		isFocus: false
	};
	let input_value: Input = {
		target: undefined,
		isWriteMode: false,
		value: '',
		isFocus: false
	};

	function validateNameInput() {
		let name = inputName.value.toLowerCase();
		if (!isVarNameValid(inputName.value)) return;
		if ($variable_names.has(name)) $variable_names.delete(name);
		$variable_names.add(name);

		// TODO backend
		variable.name = name;
		// $variables.set(variable.id, variable); // TODO backend
	}

	function validateVarInput() {
		console.log('validateVarInput');
		// TODO
	}

	function isEmpty(str: string): boolean {
		return str.length == 0;
	}

	async function varAlreadyExist(name: string): Promise<boolean> {
		if (variable.name == name) return false;
		// TODO ne pas fetch a chaque fois, le cache ailleurs
		const sequenceVariables = new Set(
			await invoke('get_variables_name_in_sequence', {
				sequenceId: $currentSequenceId
			})
		);
		return sequenceVariables.has(name);
	}

	async function isVarNameValid(name: string): Promise<boolean> {
		name = name.toLowerCase();
		return !isEmpty(name) && !(await varAlreadyExist(name));
	}

	function isVarValValid(value: string): boolean {
		// TODO
		return true;
	}
</script>

<div
	class="mx-5 flex flex-row items-center justify-center rounded-3xl border-2 border-white bg-base-100 p-2"
>
	<TextInput
		bind:input={inputName}
		value={variable.name}
		giveInput={validateNameInput}
		isInputValid={isVarNameValid}
	/>

	<MultiInput
		bind:input={input_value}
		bind:value={variable.input}
		{actionId}
		giveInput={validateVarInput}
		isInputValid={isVarValValid}
	/>
</div>
