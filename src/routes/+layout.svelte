<script>
	import '../app.css';
	import { register, unregisterAll } from '@tauri-apps/api/globalShortcut';
	import { onDestroy, onMount } from 'svelte';
	import { exit, relaunch } from '@tauri-apps/api/process';
	import { message } from '@tauri-apps/api/dialog';

	onMount(async () => {
		await register('CommandOrControl+Escape', async () => await exit(0));
		await register('CommandOrControl+R', async () => await relaunch());
		await register(
			'CommandOrControl+S',
			async () =>
				await message('You cannot Save yet', {
					title: 'Variable value',
					type: 'error'
				})
		);
	});

	onDestroy(async () => {
		await unregisterAll();
	});
</script>

<slot />
