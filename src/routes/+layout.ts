import { invoke } from '@tauri-apps/api';

export const ssr = false;
export const prerender = true;
export const csr = true;

export async function load() {
	let mainSequence: any = await invoke('get_sequence_by_id', { sequenceId: 0 });
	mainSequence.actions = [...mainSequence.actions, { id: 100, data: null }];
	console.log(mainSequence);
	return {
		mainSequence
	};
}
