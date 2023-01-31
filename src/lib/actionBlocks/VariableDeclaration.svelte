<script lang="ts">
  import { Variable, variables, variable_names } from "$lib/class/variable";
  import { Input, Vec2 } from "$lib/class";
  import TextInput from "./TextInput.svelte";
  import { onMount } from "svelte";
  import MulitInput from "./MulitInput.svelte";
  import { variableDeclarationSize } from "./defaultValues";
  import { isDialogInputOpen } from "$lib/store/userInputs";

  export let variable: Variable;
  let variable_value: any = "";

  let input_size: Vec2 = new Vec2(
    variableDeclarationSize.x / 2 - 10,
    variableDeclarationSize.y - 20
  );
  let inputName: Input = new Input(undefined, false, "");
  let input_value: Input = {
    target: undefined,
    isWriteMode: false,
    value: "",
    isFocus: false,
    name: "",
  };

  onMount(() => {
    console.log("mount", variable);
    if (!isVarNameValid(variable.name)) {
      inputName.isWriteMode = true;
      $isDialogInputOpen = true;
    }
  });

  function onWindowClick(e: any) {
    if (inputName.isWriteMode && !inputName.target.contains(e.target)) {
      inputName.isWriteMode = false;
    }
  }

  function onKeyDown(e: any) {
    // TODO remap + backend
    if (e.repeat) return;

    if (e.key == "Enter") {
      if (!isVarNameValid(inputName.value)) return;
      if (!isVarValValid(input_value.value)) return;
      let old_name = variable.name;
      if ($variable_names.has(old_name)) {
        $variable_names.delete(old_name);
      }
      if (inputName.isWriteMode) variable.name = inputName.value;
      if (input_value.isWriteMode) variable_value = input_value.value;
      $isDialogInputOpen = false;
      inputName.isWriteMode = false;
      input_value.isWriteMode = false;
      $variable_names.add(variable.name.toLowerCase());
      $variables.set(variable.id, variable);
      $variable_names = $variable_names;
    }
  }

  function isVarNameValid(name: string): boolean {
    // TODO ask rust ?
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
    bind:input={inputName}
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
