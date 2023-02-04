<script lang="ts">
	import type { Input } from '$lib/class/input';
	import { variables } from '$lib/class/variable';
	import type { Vec2 } from '$lib/class/vec2';
	import { isDialogInputOpen } from '$lib/store';
	import { register } from '@tauri-apps/api/globalShortcut';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { message } from '@tauri-apps/api/dialog';
	import DialogInput from './DialogInput.svelte';

	export let input_size: Vec2;
	export let value: any;
	export let input: Input;
	export let isInputValid: Function;
	export let actionId: number;
	export let giveInput: Function;

	let activateInput: any;
	let mode = 'text';
	let border = 'border-none';
	let draggover = false;
	//$: {
	//  input.target?.focus();
	//}

	onMount(async () => {
		console.log('multiinput mount value: ', value);
		/* TODO : mettre les shortcuts dans un composant input
    await register("Escape", () => {
      console.log("escape from input");
      input.isWriteMode = false;
      $isDialogInputOpen = input.isWriteMode;
    });

    await register("Enter", () => {
      if (!isInputValid(input.value)) return;
      $isDialogInputOpen = false;
      input.isWriteMode = false;
      value = input.value;
    });
    */
	});

	async function setDataFromDrop(data: any) {
		if (data.VariableDeclaration) {
			mode = 'var';
			border = 'border-2 border-success';
			input.value = data.VariableDeclaration.name;
		} else if (data.FunctionCall) {
			mode = 'func'; // Donc ici c'est un function call
			border = 'border-2 border-warning';
			input.value = data.FunctionCall.name;
		} else {
			mode = 'text';
			border = 'border-none';
		}
	}

	async function onDrop(event: any) {
		event.preventDefault();
		draggover = false;
		const json = event?.dataTransfer?.getData('dragDatas');
		if (!json) {
			return;
		}
		const data = JSON.parse(json);
		console.log('drop on input', data);
		if (data.actionId == actionId) {
			await message('You cannot drop on yourself', {
				title: 'Variable value',
				type: 'error'
			});
		} else {
			console.log('drop on input', data);
			await setDataFromDrop(data.data);
		}
	}

	function getPrintableValue() {}
</script>

<div
	on:dragenter={(event) => {
		draggover = true;
		console.log('dragenter', event);
	}}
	on:dragleave={(event) => {
		draggover = false;
	}}
	on:drop={onDrop}
	ondragover="return false"
>
	<button
		disabled={draggover}
		class="btn bg-inherit {border}"
		style:width="{input_size.x}px"
		style:height="{input_size.y}px"
		on:click={async () => {
			await activateInput();
		}}
	>
		{#if mode == 'var'}
			$[{input.value}]
		{:else if mode == 'func'}
			{input.value}()
		{:else if mode == 'text'}
			"{input.value}"
		{/if}
	</button>
</div>

<DialogInput bind:activateInput {value} {giveInput} {isInputValid} bind:input />
