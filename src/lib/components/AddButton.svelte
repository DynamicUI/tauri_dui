<script lang="ts">
	export let dataTip: string | null = null;
	export let onClick: Function;
	export let bgColor: string = 'bg-white';
</script>

{#if dataTip != null}
	<div class="tooltip" data-tip={dataTip}>
		<button class="btn" on:click={onClick}>
			<span class="badge {bgColor}" />
		</button>
	</div>
{:else}
	<button class="btn" on:click={onClick}>
		<span class="badge {bgColor}" />
	</button>
{/if}
