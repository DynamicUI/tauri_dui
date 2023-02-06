<script lang="ts">
	import { currentSequenceId } from '$lib/store';
	import { KeyBindings } from '$lib/components';
	import { Sequencer } from '$lib/actionBlocks';

	export let data: any;

	function isCurrentSequenceMain(): boolean {
		return $currentSequenceId === 0;
	}

	const currentSequence = null; // TODO
</script>

<KeyBindings />
{#if isCurrentSequenceMain()}
	<Sequencer sequence={data.mainSequence} />
{:else}
	<!--	<Sequencer sequence={currentSequence} />-->
{/if}
