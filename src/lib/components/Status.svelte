<script lang="ts">
	import { Inbox } from '@lucide/svelte';
	let {
		loading = false,
		error = '',
		empty = false,
		emptyText = 'Nothing here yet.'
	} = $props<{ loading?: boolean; error?: string; empty?: boolean; emptyText?: string }>();
</script>

{#if loading}<div class="loading">
		<div>
			<div class="spinner"></div>
			Gathering your numbers…
		</div>
	</div>
{:else if error}<div class="error" role="alert">{error}</div>
{:else if empty}<div class="empty">
		<div>
			<Inbox size={28} />
			<p>{emptyText}</p>
		</div>
	</div>{/if}
