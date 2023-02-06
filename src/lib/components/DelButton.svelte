<script lang="ts">
	export let onClick: any;
	export let id;
</script>

<button class="btn btn-circle bg-red-400 hover:bg-red-700" on:click={onClick}>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		class="h-6 w-6 text-black"
		fill="none"
		viewBox="0 0 24 24"
		stroke="currentColor"
	>
		<path
			stroke-linecap="round"
			stroke-linejoin="round"
			stroke-width="2"
			d="M6 18L18 6M6 6l12 12"
		/>
	</svg>
</button>
