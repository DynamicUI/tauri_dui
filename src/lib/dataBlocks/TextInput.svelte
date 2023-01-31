<script lang="ts">
  import type { Input } from "$lib/class/input";
  import type { Vec2 } from "$lib/class/vec2";
  import { isDialogInputOpen } from "$lib/store";
  import { register, unregister } from "@tauri-apps/api/globalShortcut";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  export let input_size: Vec2;
  export let value: string;
  export let input: Input;
  export let isInputValid: Function;
  export let giveInput: Function;

  $: if (input?.target) {
    input.target.focus();
  }
  onMount(async () => {
    if (!isInputValid(value)) {
      input.isWriteMode = true;
      $isDialogInputOpen = true;
    }
  });

  async function activateInput() {
    input.isWriteMode = true;
    $isDialogInputOpen = true;
    input.value = value;
    await register("Escape", disableInput);
    await register("Enter", validateInput);
  }

  async function validateInput() {
    if (!isInputValid(input.value)) return;
    giveInput(input.value);
    disableInput();
  }

  async function disableInput() {
    input.isWriteMode = false;
    $isDialogInputOpen = false;
    await unregister("Escape");
    await unregister("Enter");
  }

  async function onWindowClick(event: any) {
    if (input.isWriteMode && !input.target.contains(event.target)) {
      //await disableInput();
    }
  }
</script>

<button
  class="btn bg-inherit border-none"
  style:width="{input_size.x}px"
  style:height="{input_size.y}px"
  on:click={activateInput}
>
  {value}
</button>

{#if input.isWriteMode}
  <div
    class="modal modal-middle modal-open"
    transition:fade={{ duration: 100 }}
  >
    <div class="modal-box">
      <input
        transition:fade
        bind:value={input.value}
        bind:this={input.target}
        type="text"
        class:border-error={!isInputValid(input.value)}
        class="input input-bordered w-full border-error"
        placeholder="Variable name"
      />
    </div>
  </div>
{/if}
