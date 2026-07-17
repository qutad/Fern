<script lang="ts">
	import Modal from './Modal.svelte';
	import type { Transaction, TransactionType } from '$lib/types/domain';
	import { appState } from '$lib/stores/app.svelte';
	import { untrack } from 'svelte';

	let { item, close, save } = $props<{
		item: Transaction | null;
		close: () => void;
		save: (value: Transaction | Omit<Transaction, 'id'>) => Promise<void>;
	}>();
	const initial = untrack(() => item);
	let type = $state<TransactionType>(initial?.type ?? 'expense');
	let description = $state(initial?.description ?? '');
	let amount = $state(initial?.amount ?? 0);
	let date = $state(initial?.date ?? new Date().toISOString().slice(0, 10));
	let category = $state(initial?.category ?? 'Groceries');
	let account = $state(initial?.account ?? 'Checking');
	let note = $state(initial?.note ?? '');
	let busy = $state(false);

	async function submit(e: SubmitEvent) {
		e.preventDefault();
		busy = true;
		const value = {
			description,
			amount: Number(amount),
			date,
			category: type === 'transfer' ? 'Transfer' : category,
			account,
			note,
			type
		};
		await save(item ? { ...value, id: item.id } : value);
		busy = false;
	}
</script>

<Modal title={item ? 'Edit transaction' : 'Add transaction'} {close}>
	<form onsubmit={submit}>
		<div class="form-grid">
			<label
				>Type<select class="field" bind:value={type}
					><option value="expense">Expense</option><option value="income">Income</option><option
						value="transfer">Transfer</option
					></select
				></label
			>
			<label>Date<input class="field" type="date" bind:value={date} required /></label>
			<label class="full"
				>Description<input
					class="field"
					bind:value={description}
					placeholder="Coffee, rent, salary…"
					required
				/></label
			>
			<label
				>Amount<input
					class="field"
					type="number"
					min="0.01"
					step="0.01"
					bind:value={amount}
					required
				/></label
			>
			<label
				>Category<select class="field" bind:value={category} disabled={type === 'transfer'}
					>{#each appState.bootstrap?.categories ?? [] as name (name)}<option>{name}</option
						>{/each}</select
				></label
			>
			<label
				>Account<select class="field" bind:value={account}
					>{#each appState.bootstrap?.accounts ?? [] as name (name)}<option>{name}</option
						>{/each}</select
				></label
			>
			<label>Note<input class="field" bind:value={note} placeholder="Optional" /></label>
		</div>
		<div class="modal-actions">
			<button class="button" type="button" onclick={close}>Cancel</button><button
				class="button primary"
				disabled={busy}>{busy ? 'Saving…' : 'Save transaction'}</button
			>
		</div>
	</form>
</Modal>
