<script lang="ts">
  import type { Box } from "$lib/class/box";
  import { Input } from "$lib/class/input";
  import { Vec2 } from "$lib/class/vec2";
  import { onMount } from "svelte";
  import { Function_ } from "$lib/class/function";
  import TextInput from "./box/TextInput.svelte";
  import { isKeyboardOpen } from "$lib/store";
  import { builtins, getParamNames } from "$lib/builtins";
  import MulitInput from "./box/MulitInput.svelte";

  export let function_: Function_;

  let input_name: Input = Input.default("name");

  onMount(() => {
    input_name.isWriteMode = true;
    $isKeyboardOpen = true;
  });

  function onKeyDown(e: any) {
    if (e.repeat) return;

    if (e.key == "Escape") {
      if (!doesFnExist(function_.name)) return;
      $isKeyboardOpen = false;
      input_name.isWriteMode = false;
    }

    if (e.key == "Enter") {
      if (!doesFnExist(input_name.value)) return;
      if (input_name.isWriteMode) function_.name = input_name.value;
      $isKeyboardOpen = false;
      input_name.isWriteMode = false;
      function_.args = Function_.fromBuiltIn(function_.name) || [];
    }
  }

  function doesFnExist(name: string): boolean {
    return builtins.has(name);
  }
</script>

<div
  style="width:{box.size.x}px; height:{box.size.y}px;"
  class="bg-base-100 flex flex-row rounded-3xl mx-5 justify-center items-center"
>
  <TextInput
    bind:value={function_.name}
    bind:input_size
    bind:input={input_name}
    isInputValid={doesFnExist}
  />

  {#each function_.args as arg}
    <MulitInput
      bind:input={arg.input}
      bind:input_size
      bind:value={arg.input.value}
      id={""}
      isInputValid={function () {
        return true;
      }}
    />
  {/each}
</div>

<svelte:window on:keydown={onKeyDown} />
