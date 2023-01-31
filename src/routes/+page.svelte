<script lang="ts">
  import { currentSequenceId } from "$lib/stores";
  import KeyBindings from "./KeyBindings.svelte";
  import Sequencer from "./Sequencer.svelte";

  export let data: any;
</script>

{#if $currentSequenceId == 0}
  <div class="w-full h-screen flex justify-center items-center ">
    <KeyBindings />
      <Sequencer sequence={data.mainSequence} />
    <div class="w-96 m-10" />
  </div>
{:else}
  <div class="w-full h-screen flex justify-center items-center ">
    <!--TODO-->
    wtf
  </div>
{/if}
