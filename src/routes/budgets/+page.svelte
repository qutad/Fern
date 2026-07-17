<script lang="ts">
	import { Pencil, Plus, Trash2 } from '@lucide/svelte';
	import { api } from '$lib/api';
	import Modal from '$lib/components/Modal.svelte';
	import MonthPicker from '$lib/components/MonthPicker.svelte';
	import Status from '$lib/components/Status.svelte';
	import { appState } from '$lib/stores/app.svelte';
	import type { Budget } from '$lib/types/domain';
	import { formatMoney, percent } from '$lib/utils/format';

	let items = $state<Budget[]>([]),
		loading = $state(true),
		error = $state(''),
		open = $state(false),
		editing = $state<Budget | null>(null),
		requestedMonth = $state('');
	let category = $state('Groceries'),
		limit = $state(500);
	const currency = $derived(appState.bootstrap?.settings.currency ?? 'USD');
	let total = $derived(items.reduce((sum, b) => sum + b.limit, 0));
	let spent = $derived(items.reduce((sum, b) => sum + b.spent, 0));
	async function load(month: string) {
		requestedMonth = month;
		loading = true;
		try {
			items = await api.listBudgets(month);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Could not load budgets';
		} finally {
			loading = false;
		}
	}
	function show(item: Budget | null = null) {
		editing = item;
		category = item?.category ?? 'Groceries';
		limit = item?.limit ?? 500;
		open = true;
	}
	async function save(e: SubmitEvent) {
		e.preventDefault();
		await api.upsertBudget({
			id: editing?.id ?? '',
			category,
			limit: Number(limit),
			spent: editing?.spent ?? 0,
			month: appState.month
		});
		open = false;
		await load(appState.month);
	}
	async function remove(id: string) {
		if (!confirm('Remove this budget?')) return;
		await api.deleteBudget(id);
		items = items.filter((b) => b.id !== id);
	}
	$effect(() => {
		if (requestedMonth !== appState.month) void load(appState.month);
	});
</script>

<svelte:head><title>Budgets · Fern</title></svelte:head>
<header class="page-head">
	<div>
		<span class="eyebrow">Monthly intentions</span>
		<h1>Budgets</h1>
		<p class="muted">A flexible plan, not a punishment.</p>
	</div>
	<div class="head-actions">
		<MonthPicker /><button class="button primary" onclick={() => show()}
			><Plus size={17} /> New budget</button
		>
	</div>
</header>
{#if !loading && !error}<div class="card" style="margin-bottom:18px">
		<div class="budget-values">
			<div>
				<span class="muted">Monthly plan</span><br /><strong>{formatMoney(total, currency)}</strong>
			</div>
			<div style="text-align:right">
				<span class="muted">Available</span><br /><strong
					>{formatMoney(total - spent, currency)}</strong
				>
			</div>
		</div>
		<div class="progress"><div style={`width:${Math.min(percent(spent, total), 100)}%`}></div></div>
		<small class="muted">{percent(spent, total)}% of your planned spending used</small>
	</div>{/if}
{#if loading || error}<Status {loading} {error} />{:else if !items.length}<div class="card">
		<Status empty emptyText="Create your first category budget." />
	</div>{:else}<section class="grid budget-grid">
		{#each items as item (item.id)}
			<div class="card budget-card">
				<div class="card-head">
					<div>
						<span class="eyebrow">{percent(item.spent, item.limit)}% used</span>
						<h2>{item.category}</h2>
					</div>
					<div>
						<button class="icon-btn" aria-label={`Edit ${item.category}`} onclick={() => show(item)}
							><Pencil size={16} /></button
						>
					</div>
				</div>
				<div class="budget-values">
					<strong>{formatMoney(item.spent, currency)}</strong><span class="muted"
						>of {formatMoney(item.limit, currency)}</span
					>
				</div>
				<div class="progress">
					<div
						class:warn={percent(item.spent, item.limit) >= 75}
						class:over={percent(item.spent, item.limit) >= 100}
						style={`width:${Math.min(percent(item.spent, item.limit), 100)}%`}
					></div>
				</div>
				<small class="muted"
					>{formatMoney(Math.max(item.limit - item.spent, 0), currency)} remaining</small
				>
			</div>
		{/each}
	</section>{/if}
{#if open}<Modal title={editing ? 'Edit budget' : 'New budget'} close={() => (open = false)}
		><form onsubmit={save}>
			<div class="form-grid">
				<label
					>Category<select class="field" bind:value={category}
						>{#each appState.bootstrap?.categories ?? [] as name (name)}<option>{name}</option
							>{/each}</select
					></label
				><label
					>Monthly limit<input
						class="field"
						type="number"
						min="1"
						step="1"
						bind:value={limit}
						required
					/></label
				>
			</div>
			<div class="modal-actions">
				{#if editing}<button
						type="button"
						class="button danger"
						onclick={() => {
							void remove(editing!.id);
							open = false;
						}}><Trash2 size={16} /> Delete</button
					>{/if}<button type="button" class="button" onclick={() => (open = false)}>Cancel</button
				><button class="button primary">Save budget</button>
			</div>
		</form></Modal
	>{/if}
