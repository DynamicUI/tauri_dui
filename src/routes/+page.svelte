<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	onMount(async () => {
		await goto('/sequence');
	});
</script>

<a href="/sequence"> GO </a>
