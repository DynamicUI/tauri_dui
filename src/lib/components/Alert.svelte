<script lang="ts">
	import { fade } from 'svelte/transition';
	export let msg: string;
	export let enabled: boolean;
	$: {
		if (enabled) {
			setTimeout(() => {
				enabled = false;
			}, 2000);
		}
	}

	// TODO : juste use tauri notification api ???
</script>

{#if enabled}
	<div
		transition:fade={{ duration: 180 }}
		class="alert alert-error absolute top-8 left-[38vw] w-96 shadow-lg"
	>
		<div>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="h-6 w-6 flex-shrink-0 stroke-current"
				fill="none"
				viewBox="0 0 24 24"
				><path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
				/></svg
			>
			<span>{msg}</span>
		</div>
	</div>
{/if}
