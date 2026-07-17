<script lang="ts">
	import { X } from '@lucide/svelte';
	let { title, close, children } = $props<{
		title: string;
		close: () => void;
		children: import('svelte').Snippet;
	}>();
	function keydown(event: KeyboardEvent) {
		if (event.key === 'Escape') close();
	}
</script>

<svelte:window onkeydown={keydown} />
<div
	class="modal-backdrop"
	role="presentation"
	onclick={(e) => e.target === e.currentTarget && close()}
>
	<div class="modal" role="dialog" aria-modal="true" aria-labelledby="modal-title">
		<div class="modal-head">
			<h2 id="modal-title">{title}</h2>
			<button class="icon-btn" aria-label="Close dialog" onclick={close}><X size={19} /></button>
		</div>
		{@render children()}
	</div>
</div>
