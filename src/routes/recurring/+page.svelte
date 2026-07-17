<script lang="ts">
	import { CalendarDays, MoreHorizontal, Plus, Trash2 } from '@lucide/svelte';
	import { api } from '$lib/api';
	import RecurringModal from '$lib/components/RecurringModal.svelte';
	import Status from '$lib/components/Status.svelte';
	import { appState } from '$lib/stores/app.svelte';
	import type { RecurringPayment } from '$lib/types/domain';
	import { formatDate, formatMoney } from '$lib/utils/format';
	let items = $state<RecurringPayment[]>([]),
		loading = $state(true),
		error = $state(''),
		open = $state(false),
		editing = $state<RecurringPayment | null>(null);
	const currency = $derived(appState.bootstrap?.settings.currency ?? 'USD');
	async function load() {
		loading = true;
		try {
			items = await api.listRecurring();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Could not load recurring items';
		} finally {
			loading = false;
		}
	}
	async function save(value: RecurringPayment | Omit<RecurringPayment, 'id'>) {
		if ('id' in value) await api.updateRecurring(value);
		else await api.createRecurring(value);
		open = false;
		await load();
	}
	async function remove(id: string) {
		if (!confirm('Delete this recurring item?')) return;
		await api.deleteRecurring(id);
		items = items.filter((i) => i.id !== id);
	}
	$effect(() => {
		void load();
	});
</script>

<svelte:head><title>Recurring · Fern</title></svelte:head>
<header class="page-head">
	<div>
		<span class="eyebrow">The rhythm of money</span>
		<h1>Recurring</h1>
		<p class="muted">Subscriptions, bills, and income before they arrive.</p>
	</div>
	<button
		class="button primary"
		onclick={() => {
			editing = null;
			open = true;
		}}><Plus size={17} /> Add recurring</button
	>
</header>
{#if !loading && !error}<div class="card" style="margin-bottom:18px">
		<div class="card-head" style="margin:0">
			<div>
				<span class="eyebrow">Next 30 days</span>
				<div class="serif" style="font-size:1.8rem">
					{formatMoney(
						items.filter((i) => i.active && i.type === 'expense').reduce((s, i) => s + i.amount, 0),
						currency
					)}
				</div>
			</div>
			<CalendarDays size={32} color="var(--coral)" />
		</div>
	</div>{/if}
{#if loading || error}<Status {loading} {error} />{:else if !items.length}<div class="card">
		<Status empty emptyText="No repeating activity yet." />
	</div>{:else}<section class="grid recurring-grid">
		{#each items as item (item.id)}<article class="card recurring-card">
				<span class="category-icon">{item.name[0]}</span>
				<div class="grow">
					<h2>{item.name}</h2>
					<p>
						<span class:inactive={!item.active} class="status-dot"></span>{item.active
							? 'Active'
							: 'Paused'} · {item.frequency} · next {formatDate(item.nextDate)}
					</p>
				</div>
				<div>
					<div class:income={item.type === 'income'} class="amount">
						{item.type === 'income' ? '+' : '−'}{formatMoney(item.amount, currency)}
					</div>
					<div style="display:flex;justify-content:flex-end">
						<button
							class="icon-btn"
							aria-label={`Edit ${item.name}`}
							onclick={() => {
								editing = item;
								open = true;
							}}><MoreHorizontal size={18} /></button
						><button
							class="icon-btn"
							aria-label={`Delete ${item.name}`}
							onclick={() => remove(item.id)}><Trash2 size={15} /></button
						>
					</div>
				</div>
			</article>{/each}
	</section>{/if}
{#if open}<RecurringModal item={editing} close={() => (open = false)} {save} />{/if}
