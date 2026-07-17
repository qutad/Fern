<script lang="ts">
	import { resolve } from '$app/paths';
	import { ArrowDownLeft, ArrowUpRight, Landmark, PiggyBank } from '@lucide/svelte';
	import { api } from '$lib/api';
	import DonutChart from '$lib/charts/DonutChart.svelte';
	import TrendChart from '$lib/charts/TrendChart.svelte';
	import MonthPicker from '$lib/components/MonthPicker.svelte';
	import Status from '$lib/components/Status.svelte';
	import { appState } from '$lib/stores/app.svelte';
	import type { DashboardData } from '$lib/types/domain';
	import { formatDate, formatMoney } from '$lib/utils/format';

	let data = $state<DashboardData | null>(null);
	let loading = $state(true);
	let error = $state('');
	let requestedMonth = $state('');
	const currency = $derived(appState.bootstrap?.settings.currency ?? 'USD');

	async function load(month: string) {
		requestedMonth = month;
		loading = true;
		error = '';
		try {
			data = await api.getDashboard(month);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Could not load dashboard';
		} finally {
			loading = false;
		}
	}

	$effect(() => {
		if (appState.month !== requestedMonth) void load(appState.month);
	});
</script>

<svelte:head><title>Dashboard · Fern</title></svelte:head>
<header class="page-head">
	<div>
		<span class="eyebrow">Financial field notes</span>
		<h1>Good morning.</h1>
		<p class="muted">Here’s the shape of your month so far.</p>
	</div>
	<MonthPicker />
</header>

{#if loading || error || !data}<Status {loading} {error} />
{:else}
	<section class="grid metrics" aria-label="Monthly summary">
		<div class="card metric">
			<div class="metric-label"><span>Total balance</span><Landmark size={17} /></div>
			<div class="metric-value">{formatMoney(data.balance, currency)}</div>
			<span class="delta">↑ 4.2% this month</span>
		</div>
		<div class="card metric">
			<div class="metric-label"><span>Income</span><ArrowDownLeft size={17} /></div>
			<div class="metric-value">{formatMoney(data.income, currency)}</div>
			<span class="delta">On track</span>
		</div>
		<div class="card metric">
			<div class="metric-label"><span>Expenses</span><ArrowUpRight size={17} /></div>
			<div class="metric-value">{formatMoney(data.expenses, currency)}</div>
			<span class="delta down">8 days remaining</span>
		</div>
		<div class="card metric">
			<div class="metric-label"><span>Savings rate</span><PiggyBank size={17} /></div>
			<div class="metric-value">{data.savingsRate}%</div>
			<span class="delta">↑ 3% from last month</span>
		</div>
	</section>
	<section class="grid dashboard-grid">
		<div class="card">
			<div class="card-head">
				<div>
					<span class="eyebrow">Six month view</span>
					<h2>Cash flow</h2>
				</div>
				<div class="legend">
					<span><i style="background:#557d60"></i>Income</span><span
						><i style="background:#d96548"></i>Expenses</span
					>
				</div>
			</div>
			<TrendChart data={data.monthlyTrend} />
		</div>
		<div class="card">
			<div class="card-head">
				<div>
					<span class="eyebrow">Where it went</span>
					<h2>Spending</h2>
				</div>
			</div>
			<DonutChart data={data.categorySpend} {currency} />
		</div>
		<div class="card">
			<div class="card-head">
				<h2>Recent activity</h2>
				<a class="muted" href={resolve('/transactions')}>View all →</a>
			</div>
			<div class="transaction-list">
				{#each data.recentTransactions as item (item.id)}<div class="transaction-row">
						<span class="category-icon">{item.category[0]}</span>
						<div><strong>{item.description}</strong><small>{formatDate(item.date)}</small></div>
						<small>{item.category}</small><span class:income={item.type === 'income'} class="amount"
							>{item.type === 'income' ? '+' : item.type === 'expense' ? '−' : ''}{formatMoney(
								item.amount,
								currency
							)}</span
						>
					</div>{/each}
			</div>
		</div>
		<div class="card">
			<div class="card-head">
				<h2>Coming up</h2>
				<a class="muted" href={resolve('/recurring')}>Manage →</a>
			</div>
			<div class="upcoming-list">
				{#each data.upcoming as item (item.id)}<div class="upcoming-item">
						<span class="date-tile"
							>{formatDate(item.nextDate, { month: 'short' }).split(' ')[0]}<b
								>{new Date(`${item.nextDate}T12:00:00`).getDate()}</b
							></span
						>
						<div><strong>{item.name}</strong><br /><small class="muted">{item.account}</small></div>
						<span class="amount">{formatMoney(item.amount, currency)}</span>
					</div>{/each}
			</div>
		</div>
	</section>
{/if}
