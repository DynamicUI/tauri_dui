<script lang="ts">
  import type { Input } from "$lib/class/input";
  import type { Vec2 } from "$lib/class/vec2";
  import { isDialogInputOpen } from "$lib/store";
  import { fade } from "svelte/transition";

  export let input_size: Vec2;
  export let value: string;
  export let input: Input;
  export let isInputValid: Function;

  $: {
    input.target?.focus();
  }

  function onKeyDown(e: any) {
    if (e.repeat) return;
    console.log(e.key);

    if (e.key == "Escape") {
      input.isWriteMode = false;
      $isDialogInputOpen = input.isWriteMode;
    }
  }
</script>

<button
  class="btn bg-inherit border-none"
  style:width="{input_size.x}px"
  style:height="{input_size.y}px"
  on:click={() => {
    input.value = value;
    input.isWriteMode = true;
  }}
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

<svelte:window on:keydown={onKeyDown} />
