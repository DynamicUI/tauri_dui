<script lang="ts">
  import { Variable, variables, variable_names } from "$lib/class/variable";
  import type { Box } from "$lib/class/box";
  import { Input } from "$lib/class/input";
  import { Vec2 } from "$lib/class/vec2";
  import TextInput from "./TextInput.svelte";
  import { onMount } from "svelte";
  import { isKeyboardOpen } from "$lib/store";
  import MulitInput from "./MulitInput.svelte";
  import { variableDeclarationSize } from "./defaultValues";

  export let variable: Variable;
  export let delBlock: Function;
  let variable_value: any = "";

  let input_size: Vec2 = new Vec2(
    variableDeclarationSize.x / 2 - 10,
    variableDeclarationSize.y - 20
  );
  let input_name: Input = Input.default("name");
  let input_value: Input = Input.default("value");
  onMount(() => {
    input_name.isWriteMode = true;
    $isKeyboardOpen = true;
  });

  function onWindowClick(e: any) {
    if (input_name.isWriteMode && !input_name.target.contains(e.target)) {
      input_name.isWriteMode = false;
    }
  }

  function onKeyDown(e: any) {
    if (e.repeat) return;

    if (e.key == "Enter") {
      if (!isVarNameValid(input_name.value)) return;
      if (!isVarValValid(input_value.value)) return;
      let old_name = variable.name;
      if ($variable_names.has(old_name)) {
        $variable_names.delete(old_name);
      }
      if (input_name.isWriteMode) variable.name = input_name.value;
      if (input_value.isWriteMode) variable_value = input_value.value;
      $isKeyboardOpen = false;
      input_name.isWriteMode = false;
      input_value.isWriteMode = false;
      $variable_names.add(variable.name.toLowerCase());
      $variables.set(variable.id, variable);
      $variable_names = $variable_names;
    }
  }

  function isVarNameValid(name: string): boolean {
    name = name.toLowerCase();
    return name.length != 0 && !$variable_names.has(name);
  }

  function isVarValValid(value: string): boolean {
    // TODO
    return true;
  }

  $: {
    if (typeof variable_value.id == "string") {
      console.log("variable", variable_value);
      variable.value = variable_value.id;
    }
  }
</script>

<div
  style:width="{variableDeclarationSize.x}px"
  style:height="{variableDeclarationSize.y}px"
  class="bg-base-100 flex flex-row rounded-3xl mx-5 justify-center items-center"
>
  <TextInput
    bind:input={input_name}
    bind:input_size
    bind:value={variable.name}
    isInputValid={isVarNameValid}
  />
  <MulitInput
    bind:input={input_value}
    bind:input_size
    bind:value={variable_value}
    bind:id={variable.id}
    isInputValid={isVarValValid}
  />
</div>

<svelte:window on:mouseup={onWindowClick} on:keydown={onKeyDown} />
