<script lang="ts">
	import DialogInput from './DialogInput.svelte';
	import type { Input } from '$lib/class/input';
	import { isDialogInputOpen } from '$lib/store';
	import { onMount } from 'svelte';

	export let value: string;
	export let input: Input;
	export let isInputValid: Function;
	export let giveInput: Function;

	let activateInput: any;

	$: if (input?.target) {
		input?.target.focus();
	}

	onMount(async () => {
		if (!(await isInputValid(value))) {
			input.isWriteMode = true;
			$isDialogInputOpen = true;
		}
	});
</script>

<button
	class="btn border-none bg-inherit"
	on:click={async () => {
		await activateInput();
	}}
>
	{value}
</button>

<DialogInput bind:activateInput {value} {giveInput} {isInputValid} bind:input />
