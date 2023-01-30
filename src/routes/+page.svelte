<script lang="ts">
  import { currentSequenceId } from "$lib/stores";
  import AddButtonGroup from "./AddButtonGroup.svelte";
  import KeyBindings from "./KeyBindings.svelte";
  import Sequencer from "./Sequencer.svelte";

  export let data: any;
</script>

{#if $currentSequenceId == 0}
  <div class="w-full h-screen flex justify-center items-center ">
    <KeyBindings />
    <div class="bg-neutral rounded-3xl">
      <Sequencer sequence={data.mainSequence} />
      <AddButtonGroup />
    </div>
    <div class="w-96 m-10" />
  </div>
{:else}
  <div class="w-full h-screen flex justify-center items-center ">
    <!--TODO-->
    wtf
  </div>
{/if}
