<script lang="ts">
	import type { Input } from '$lib/class/input';
	import { onMount } from 'svelte';
	import { message } from '@tauri-apps/api/dialog';
	import DialogInput from './DialogInput.svelte';
	import { ToggleSequence } from '$lib/actionBlocks';

	export let value: any;
	export let input: Input;
	export let isInputValid: Function;
	export let actionId: number;
	export let giveInput: Function;

	let activateInput: any;
	let mode = 'text';
	let border = 'border-none';
	let dragover = false;
	//$: {
	//  input.target?.focus();
	//}

	onMount(async () => {
		console.log('multiinput mount value: ', value);
		if (value == undefined) {
			mode = 'none';
		}
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
		dragover = false;
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
</script>

<div
	on:dragenter={(event) => {
		dragover = true;
		console.log('dragenter', event);
	}}
	on:dragleave={() => {
		dragover = false;
	}}
	on:drop={onDrop}
	ondragover="return false"
	class="p-5"
>
	{#if mode === 'none'}
		<div class="grid min-w-fit grid-cols-2 {dragover ? 'bg-black' : ''}">
			<button
				class="btn border-none bg-inherit"
				on:click={async () => {
					await activateInput();
					mode = 'text';
					input.value = '';
				}}
				>""
			</button>
			<button
				class="btn border-none bg-inherit"
				on:click={async () => {
					//await activateInput();
				}}
			>
				x()
			</button>
			<button
				class="btn border-none bg-inherit"
				on:click={() => {
					mode = 'lambda';
				}}
			>
				{'{}'}
			</button>
		</div>
	{:else if mode === 'text' || mode === 'var' || mode === 'func'}
		<button
			disabled={dragover}
			class="btn bg-inherit {border}"
			on:click={async () => {
				await activateInput();
			}}
		>
			{#if mode === 'var'}
				$[{input.value}]
			{:else if mode === 'func'}
				{input.value}()
			{:else if mode === 'text'}
				"{input.value}"
			{/if}
		</button>
	{:else if mode === 'lambda'}
		<ToggleSequence sequence={{ actions: [], id: 43 }} />
	{/if}
</div>

<DialogInput bind:activateInput {value} {giveInput} {isInputValid} bind:input />
