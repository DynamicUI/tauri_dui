import { invoke } from "@tauri-apps/api";
import { currentSequenceId } from "$lib/stores";

export const ssr = false;
export const prerender = true;
export const csr = true;

export async function load() {
  return {
    mainSequence: await invoke("get_sequence_by_id", { sequenceId: 0 }),
  };
}
