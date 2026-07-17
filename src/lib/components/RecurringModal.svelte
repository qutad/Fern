<script lang="ts">
	import Modal from './Modal.svelte';
	import { appState } from '$lib/stores/app.svelte';
	import type { RecurringPayment } from '$lib/types/domain';
	import { untrack } from 'svelte';
	let { item, close, save } = $props<{
		item: RecurringPayment | null;
		close: () => void;
		save: (value: RecurringPayment | Omit<RecurringPayment, 'id'>) => Promise<void>;
	}>();
	const initial = untrack(() => item);
	let name = $state(initial?.name ?? ''),
		amount = $state(initial?.amount ?? 0),
		category = $state(initial?.category ?? 'Utilities'),
		account = $state(initial?.account ?? 'Checking');
	let frequency = $state<RecurringPayment['frequency']>(initial?.frequency ?? 'monthly'),
		nextDate = $state(initial?.nextDate ?? new Date().toISOString().slice(0, 10));
	let type = $state<RecurringPayment['type']>(initial?.type ?? 'expense'),
		active = $state(initial?.active ?? true),
		busy = $state(false);
	async function submit(e: SubmitEvent) {
		e.preventDefault();
		busy = true;
		const value = {
			name,
			amount: Number(amount),
			category,
			account,
			frequency,
			nextDate,
			type,
			active
		};
		await save(item ? { ...value, id: item.id } : value);
		busy = false;
	}
</script>

<Modal title={item ? 'Edit recurring item' : 'Add recurring item'} {close}
	><form onsubmit={submit}>
		<div class="form-grid">
			<label class="full"
				>Name<input
					class="field"
					bind:value={name}
					placeholder="Rent, subscription, retainer…"
					required
				/></label
			>
			<label
				>Amount<input
					class="field"
					type="number"
					min=".01"
					step=".01"
					bind:value={amount}
					required
				/></label
			><label
				>Type<select class="field" bind:value={type}
					><option value="expense">Expense</option><option value="income">Income</option></select
				></label
			>
			<label
				>Category<select class="field" bind:value={category}
					>{#each appState.bootstrap?.categories ?? [] as option (option)}<option>{option}</option
						>{/each}</select
				></label
			><label
				>Account<select class="field" bind:value={account}
					>{#each appState.bootstrap?.accounts ?? [] as option (option)}<option>{option}</option
						>{/each}</select
				></label
			>
			<label
				>Frequency<select class="field" bind:value={frequency}
					><option value="weekly">Weekly</option><option value="monthly">Monthly</option><option
						value="quarterly">Quarterly</option
					><option value="yearly">Yearly</option></select
				></label
			><label>Next date<input class="field" type="date" bind:value={nextDate} required /></label>
			<label class="full" style="display:flex;grid-template-columns:auto 1fr;align-items:center"
				><input type="checkbox" bind:checked={active} /> Active and included in forecasts</label
			>
		</div>
		<div class="modal-actions">
			<button type="button" class="button" onclick={close}>Cancel</button><button
				class="button primary"
				disabled={busy}>{busy ? 'Saving…' : 'Save item'}</button
			>
		</div>
	</form></Modal
>
