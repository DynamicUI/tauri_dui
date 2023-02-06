<script lang="ts">
	import { fade } from 'svelte/transition';
	import { register, unregister } from '@tauri-apps/api/globalShortcut';
	import { isDialogInputOpen } from '$lib/store';

	export let input: any;
	export let isInputValid: Function;
	export let giveInput: Function;
	export let value: string;

	export async function activateInput() {
		console.log('activateInput');
		input.isWriteMode = true;
		$isDialogInputOpen = true;
		input.value = value;
		await register('Escape', disableInput);
		console.log('register escape');
		await register('Enter', validateInput);
	}

	async function validateInput() {
		if (!isInputValid(input.value)) return;
		giveInput(input.value);
		disableInput();
	}

	async function disableInput() {
		console.log('disableInput');
		input.isWriteMode = false;
		$isDialogInputOpen = false;
		await unregister('Escape');
		await unregister('Enter');
	}
</script>

{#if input.isWriteMode}
	<div class="modal modal-open modal-middle" transition:fade={{ duration: 100 }}>
		<div class="modal-box">
			<input
				transition:fade
				bind:value={input.value}
				bind:this={input.target}
				type="text"
				class:border-error={!isInputValid(input.value)}
				class="input-bordered input w-full border-error"
				placeholder="Variable name"
			/>
		</div>
	</div>
{/if}
