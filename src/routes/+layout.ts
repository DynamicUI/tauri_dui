import { invoke } from '@tauri-apps/api';

export const ssr = false;
export const prerender = true;
export const csr = true;

export async function load() {
	const mainSequence = await invoke('get_sequence_by_id', { sequenceId: 0 });
	console.log(mainSequence);
	return {
		mainSequence
	};
}
