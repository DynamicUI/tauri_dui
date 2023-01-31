<script>
  import { currentSequenceId } from "$lib/store";
  import { onMount } from "svelte";
  import "../app.css";

  onMount(() => {
    if ($currentSequenceId == -1) {
      $currentSequenceId = 0;
    }
  });
</script>

<main class="container">
  <slot />
</main>
