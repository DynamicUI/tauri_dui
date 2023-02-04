<script lang="ts">
	export let addVariableDecl: Function;
	export let addFunctionCall: Function;
</script>

<div class="btn-group flex justify-center">
	<div class="tooltip" data-tip="Add a variable declaration">
		<button class="btn" on:click={addVariableDecl}>
			<span class="badge bg-success" />
		</button>
	</div>
	<div class="tooltip" data-tip="Add a function call">
		<button class="btn" on:click={addFunctionCall}>
			<span class="badge bg-warning" />
		</button>
	</div>
	<div class="tooltip" data-tip="">
		<button class="btn">
			<span class="badge bg-error" />
		</button>
	</div>
	<div class="tooltip" data-tip="run">
		<button class="btn">
			<svg
				class="fill-white"
				width="20px"
				height="20px"
				viewBox="0 0 16 16"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					d="m 2 2.5 v 11 c 0 1.5 1.269531 1.492188 1.269531 1.492188 h 0.128907 c 0.246093 0.003906 0.488281 -0.050782 0.699218 -0.171876 l 9.796875 -5.597656 c 0.433594 -0.242187 0.65625 -0.734375 0.65625 -1.226562 c 0 -0.492188 -0.222656 -0.984375 -0.65625 -1.222656 l -9.796875 -5.597657 c -0.210937 -0.121093 -0.453125 -0.175781 -0.699218 -0.175781 h -0.128907 s -1.269531 0 -1.269531 1.5 z m 0 0"
				/>
			</svg>
		</button>
	</div>
</div>
