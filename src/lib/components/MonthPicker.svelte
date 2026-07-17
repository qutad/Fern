<script lang="ts">
	import { ChevronLeft, ChevronRight } from '@lucide/svelte';
	import { appState } from '$lib/stores/app.svelte';
	import { monthLabel, shiftMonth } from '$lib/utils/format';
</script>

<div class="month-picker" aria-label="Selected month">
	<button
		class="icon-btn"
		aria-label="Previous month"
		onclick={() => (appState.month = shiftMonth(appState.month, -1))}
		><ChevronLeft size={17} /></button
	>
	<strong>{monthLabel(appState.month)}</strong>
	<button
		class="icon-btn"
		aria-label="Next month"
		onclick={() => (appState.month = shiftMonth(appState.month, 1))}
		><ChevronRight size={17} /></button
	>
</div>
