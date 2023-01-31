<script lang="ts">
  import type { Input } from "$lib/class/input";
  import { variables } from "$lib/class/variable";
  import type { Vec2 } from "$lib/class/vec2";
  import { isDialogInputOpen } from "$lib/store";
  import { register } from "@tauri-apps/api/globalShortcut";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  export let input_size: Vec2;
  export let value: any;
  export let input: Input;
  export let isInputValid: Function;
  export let id: string;

  let mode = "text";
  let border = "border-none";
  let draggover = false;
  //$: {
  //  input.target?.focus();
  //}

  onMount(async () => {
    console.log("mulit",value)
    /* TODO : mettre les shortcuts dans un composant input
    await register("Escape", () => {
      console.log("escape from input");
      input.isWriteMode = false;
      $isDialogInputOpen = input.isWriteMode;
    });

    await register("Enter", () => {
      if (!isInputValid(input.value)) return;
      $isDialogInputOpen = false;
      input.isWriteMode = false;
      value = input.value;
    });
    */
  });

  function updateMode(id: string) {
    if (id.startsWith("var")) {
      mode = "var";
      border = "border-2 border-success";
    } else if (id.startsWith("func")) {
      mode = "func";
      border = "border-2 border-warning";
    } else if (id.startsWith("error")) {
      mode = "error";
      border = "border-2 border-danger";
    } else {
      mode = "text";
      border = "border-none";
    }
  }
</script>

<div
  on:dragenter={(event) => {
    draggover = true;
  }}
  on:dragleave={(event) => {
    draggover = false;
  }}
  on:drop={(event) => {
    event.preventDefault();
    const json = event?.dataTransfer?.getData("dragDatas");
    if (json) {
      const data = JSON.parse(json);
      if (data.id == id) {
        console.error("you can't drop on yourself");
      } else {
        input.value = data;
        if (typeof input.value.id == "string") {
          updateMode(data.id);
        } else {
          mode = "text";
        }
      }
    }
    draggover = false;
  }}
  ondragover="return false"
>
  <button
    disabled={draggover}
    class="btn bg-inherit {border}"
    style:width="{input_size.x}px"
    style:height="{input_size.y}px"
    on:click={() => {
      input.value = input.value;
      input.isWriteMode = true;
    }}
  >
    {#if mode == "var"}
      [{$variables.get(n_id).name}]
    {:else}
      {input.value}
    {/if}
  </button>
</div>

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
