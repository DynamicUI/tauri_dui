<script lang="ts">
  /*
   * The sequencer represent an array of actions that will be executed in order.
   * It can be used as lambda function, function declaration, main function, etc.
   */
  import { register, unregisterAll } from "@tauri-apps/api/globalShortcut";
  import AddButtonGroup from "./AddButtonGroup.svelte";
  import { onDestroy, onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { flip } from "svelte/animate";
  import Alert from "./Alert.svelte";
  import { isDialogInputOpen } from "$lib/store";
  import { variableDeclarationSize } from "$lib/actionBlocks/defaultValues";
  import VariableDeclaration from "$lib/actionBlocks/VariableDeclaration.svelte";

  let isSaving: boolean = false;
  onMount(async () => {
    await register("CommandOrControl+r", () => {
      //actionBlocks = [];
      // TODO ask first
      // backend
      $isDialogInputOpen = false;
    });
    await register("CommandOrControl+a", () => {
      addVariableDecl();
    });
    await register("CommandOrControl+s", () => {
      console.log("Saving");
      isSaving = true;
    });
  });
  onDestroy(async () => {
    unregisterAll();
  });

  export let sequence: any; // From rust
  let actionBlocks: any[] = [];

  function delBlock(id: String) {
    actionBlocks = actionBlocks.filter((actionBlock) => actionBlock.id != id);
  }

  function addFunctionCall() {
    console.log("add f call");
    // TODO backend
    //actionBlock = [...actionBlock, new F];
  }

  function addVariableDecl() {
    console.log("add v call");
    // TODO backend
    //actionBlock = [...actionBlock, new V];
  }
</script>

id: {sequence.id} context: {sequence.context}
<div class="bg-neutral items-center  rounded-2xl justify-center">
  <div style="width: {variableDeclarationSize.x * 1.9}px;" />
  <div class="flex  flex-col p-16 overflow-scroll h-[80vh] items-center">
    {#each sequence.actions as actionBlock (actionBlock.id)}
      <div animate:flip={{ duration: 150 }} transition:fade={{ duration: 150 }}>
        <div class="flex items-center mb-10">
          <!--
          <span
            class="badge {actionBlock instanceof Variable
              ? 'bg-success'
              : 'bg-warning'}"
            draggable="true"
            on:dragstart={(event) => {
              event?.dataTransfer?.setData(
                "dragDatas",
                JSON.stringify({ id: actionBlock.id })
              );
            }}
          />
          -->

          {#if actionBlock.data.VariableDeclaration}
            <VariableDeclaration
              bind:variable={actionBlock.data.VariableDeclaration}
            />
          {/if}

          <!--
          {#if actionBlock instanceof Variable}
            <VariableDeclaration bind:variable={actionBlock} {delBlock} />
          {:else if actionBlock instanceof Function_}
            <FunctionCall bind:function_={actionBlock} {delBlock} />
          {/if}
          -->
          <button
            class="btn btn-circle bg-red-400 hover:bg-red-700"
            on:click={() => {
              //delBlock(actionBlock.id);
            }}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-6 w-6 text-black"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              /></svg
            >
          </button>
        </div>
      </div>
    {/each}
  </div>
  <AddButtonGroup {addVariableDecl} {addFunctionCall} />
</div>

<Alert bind:enabled={isSaving} msg={"You can't save, idiot..."} />
