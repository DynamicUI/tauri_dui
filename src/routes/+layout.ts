// noinspection JSUnusedGlobalSymbols

import { invoke } from '@tauri-apps/api';
import { builtinsFunctions } from '$lib/store';

export const ssr = false;
export const prerender = true;
export const csr = true;

export async function load() {
	let mainSequence: any = await invoke('get_sequence_by_id', { sequenceId: 0 });
	const builtins: any = await invoke('get_builtins_functions_list');
	builtinsFunctions.set(builtins);
	//mainSequence.actions = [...mainSequence.actions, { id: 100, data: null }];
	return {
		mainSequence
	};
}
