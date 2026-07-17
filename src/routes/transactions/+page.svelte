<script lang="ts">
	import { MoreHorizontal, Plus, Search, Trash2 } from '@lucide/svelte';
	import { api } from '$lib/api';
	import MonthPicker from '$lib/components/MonthPicker.svelte';
	import Status from '$lib/components/Status.svelte';
	import TransactionModal from '$lib/components/TransactionModal.svelte';
	import { appState } from '$lib/stores/app.svelte';
	import type { Transaction, TransactionType } from '$lib/types/domain';
	import { formatDate, formatMoney } from '$lib/utils/format';

	let items = $state<Transaction[]>([]),
		loading = $state(true),
		error = $state(''),
		query = $state(''),
		filter = $state<'all' | TransactionType>('all');
	let editing = $state<Transaction | null>(null),
		modal = $state(false),
		requestedMonth = $state('');
	const currency = $derived(appState.bootstrap?.settings.currency ?? 'USD');
	let filtered = $derived(
		items.filter(
			(item) =>
				(filter === 'all' || item.type === filter) &&
				`${item.description} ${item.category} ${item.account}`
					.toLowerCase()
					.includes(query.toLowerCase())
		)
	);

	async function load(month: string) {
		requestedMonth = month;
		loading = true;
		try {
			items = await api.listTransactions(month);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Could not load transactions';
		} finally {
			loading = false;
		}
	}
	async function save(value: Transaction | Omit<Transaction, 'id'>) {
		if ('id' in value) await api.updateTransaction(value);
		else await api.createTransaction(value);
		modal = false;
		await load(appState.month);
	}
	async function remove(id: string) {
		if (!confirm('Delete this transaction?')) return;
		await api.deleteTransaction(id);
		items = items.filter((item) => item.id !== id);
	}
	$effect(() => {
		if (requestedMonth !== appState.month) void load(appState.month);
	});
</script>

<svelte:head><title>Transactions · Fern</title></svelte:head>
<header class="page-head">
	<div>
		<span class="eyebrow">The ledger</span>
		<h1>Transactions</h1>
		<p class="muted">Every movement, in one considered place.</p>
	</div>
	<div class="head-actions">
		<MonthPicker /><button
			class="button primary"
			onclick={() => {
				editing = null;
				modal = true;
			}}><Plus size={17} /> Add transaction</button
		>
	</div>
</header>
<div class="toolbar">
	<div class="search">
		<Search size={17} /><input
			aria-label="Search transactions"
			placeholder="Search descriptions, categories…"
			bind:value={query}
		/>
	</div>
	<select class="field" style="width:auto" aria-label="Filter by type" bind:value={filter}
		><option value="all">All activity</option><option value="expense">Expenses</option><option
			value="income">Income</option
		><option value="transfer">Transfers</option></select
	>
</div>
{#if loading || error}<Status {loading} {error} />
{:else if filtered.length === 0}<div class="card">
		<Status empty emptyText="No transactions match your filters." />
	</div>
{:else}<div class="table-wrap">
		<table>
			<thead
				><tr
					><th>Date</th><th>Description</th><th class="hide-mobile">Category</th><th
						class="hide-mobile">Account</th
					><th>Type</th><th style="text-align:right">Amount</th><th aria-label="Actions"></th></tr
				></thead
			><tbody>
				{#each filtered as item (item.id)}<tr
						><td>{formatDate(item.date)}</td><td><strong>{item.description}</strong></td><td
							class="hide-mobile">{item.category}</td
						><td class="hide-mobile muted">{item.account}</td><td
							><span class:expense={item.type === 'expense'} class="pill">{item.type}</span></td
						><td class:income={item.type === 'income'} class="amount"
							>{item.type === 'income' ? '+' : item.type === 'expense' ? '−' : ''}{formatMoney(
								item.amount,
								currency
							)}</td
						><td
							><div class="table-actions">
								<button
									class="icon-btn"
									aria-label={`Edit ${item.description}`}
									onclick={() => {
										editing = item;
										modal = true;
									}}><MoreHorizontal size={18} /></button
								><button
									class="icon-btn"
									aria-label={`Delete ${item.description}`}
									onclick={() => remove(item.id)}><Trash2 size={15} /></button
								>
							</div></td
						></tr
					>{/each}
			</tbody>
		</table>
	</div>{/if}
{#if modal}<TransactionModal item={editing} close={() => (modal = false)} {save} />{/if}
