<script lang="ts">
  import { Variable, variables, variable_names } from "$lib/class/variable";
  import { Input, Vec2 } from "$lib/class";
  import { variableDeclarationSize } from "./defaultValues";
  import { MulitInput, TextInput } from "$lib/dataBlocks";

  export let variable: Variable;

  let input_size: Vec2 = new Vec2(
    variableDeclarationSize.x / 2 - 10,
    variableDeclarationSize.y - 20
  );
  let inputName: Input = {
    target: undefined,
    isWriteMode: false,
    value: "",
    isFocus: false,
  };
  let input_value: Input = {
    target: undefined,
    isWriteMode: false,
    value: "",
    isFocus: false,
  };

  function validateNameInput() {
    let name = inputName.value.toLowerCase();
    if (!isVarNameValid(inputName.value)) return;
    if ($variable_names.has(name)) $variable_names.delete(name);
    $variable_names.add(name);

    // TODO backend
    variable.name = name;
    // $variables.set(variable.id, variable); // TODO backend
  }

  function isEmpty(str: string): boolean {
    return str.length == 0;
  }

  function varAlreadyExist(name: string): boolean {
    // TODO ask rust ?
    return $variable_names.has(name);
  }

  function isVarNameValid(name: string): boolean {
    name = name.toLowerCase();
    return !isEmpty(name) && !varAlreadyExist(name);
  }

  // TODO validateVarInput
  function isVarValValid(value: string): boolean {
    // TODO
    return true;
  }
  /*

  $: {
    if (typeof variable_value.id == "string") {
      console.log("variable", variable_value);
      variable.value = variable_value.id;
    }
  }
  */
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
    giveInput={validateNameInput}
    isInputValid={isVarNameValid}
  />

  <MulitInput
    bind:input={input_value}
    bind:input_size
    bind:value={variable.input}
    bind:id={variable.id}
    isInputValid={isVarValValid}
  />
</div>
