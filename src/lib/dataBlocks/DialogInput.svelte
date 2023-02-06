<script lang="ts">
	import { fade } from 'svelte/transition';
	import { register, unregister } from '@tauri-apps/api/globalShortcut';
	import { isDialogInputOpen } from '$lib/store';
	import { onMount } from 'svelte';

	export let input: any;
	export let isInputValid: Function;
	export let giveInput: Function;
	export let value: string;

	let invalidWarning = false;
	onMount(async () => {
		invalidWarning = !(await isInputValid(input.value));
	});

	export async function activateInput() {
		input.isWriteMode = true;
		$isDialogInputOpen = true;
		input.value = value;
		await register('Escape', disableInput);
	}

	async function validateInput() {
		if (!(await isInputValid(input.value))) {
			input.target.focus(); // FIXME: this is not working
			return;
		}
		giveInput(input.value);
		await disableInput();
	}

	async function disableInput() {
		input.isWriteMode = false;
		$isDialogInputOpen = false;
		await unregister('Escape');
	}
</script>

{#if input.isWriteMode}
	<div class="modal modal-open modal-middle" transition:fade={{ duration: 100 }}>
		<div class="modal-box">
			<form on:submit={validateInput}>
				<input
					transition:fade
					bind:value={input.value}
					bind:this={input.target}
					on:input={async () => {
						invalidWarning = !(await isInputValid(input.value));
					}}
					type="text"
					class="input-bordered input w-full"
					placeholder="Variable name"
					class:border-error={invalidWarning}
				/>
			</form>
		</div>
	</div>
{/if}
