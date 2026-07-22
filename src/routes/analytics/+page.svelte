<script lang="ts">
	import { Gauge, Receipt, TrendingUp } from '@lucide/svelte';
	import { api } from '$lib/api';
	import BarsChart from '$lib/charts/BarsChart.svelte';
	import DonutChart from '$lib/charts/DonutChart.svelte';
	import TrendChart from '$lib/charts/TrendChart.svelte';
	import MonthPicker from '$lib/components/MonthPicker.svelte';
	import Status from '$lib/components/Status.svelte';
	import { appState } from '$lib/stores/app.svelte';
	import type { AnalyticsData } from '$lib/types/domain';
	import { formatMoney } from '$lib/utils/format';
	let data = $state<AnalyticsData | null>(null),
		loading = $state(true),
		error = $state(''),
		requestedMonth = $state('');
	const currency = $derived(appState.bootstrap?.settings.currency ?? 'USD');
	async function load(month: string) {
		requestedMonth = month;
		loading = true;
		try {
			data = await api.getAnalytics(month);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Could not load analytics';
		} finally {
			loading = false;
		}
	}
	$effect(() => {
		if (requestedMonth !== appState.month) void load(appState.month);
	});
</script>

<svelte:head><title>Analytics · Fern</title></svelte:head>
<header class="page-head">
	<div>
		<span class="eyebrow">Patterns & perspective</span>
		<h1>Analytics</h1>
		<p class="muted">See the story behind the numbers.</p>
	</div>
	<MonthPicker />
</header>
{#if loading || error || !data}<Status {loading} {error} />{:else}<section
		class="grid metrics analytics-metrics"
	>
		<div class="card metric">
			<div class="metric-label">Average daily spend<Gauge size={17} /></div>
			<div class="metric-value">{formatMoney(data.averageDailySpend, currency)}</div>
			<span class="muted">Across selected month</span>
		</div>
		<div class="card metric">
			<div class="metric-label">Largest expense<Receipt size={17} /></div>
			<div class="metric-value">{formatMoney(data.largestExpense?.amount ?? 0, currency)}</div>
			<span class="muted">{data.largestExpense?.description ?? 'No expenses'}</span>
		</div>
		<div class="card metric">
			<div class="metric-label">Net cash flow<TrendingUp size={17} /></div>
			<div class="metric-value">{formatMoney(data.income - data.expenses, currency)}</div>
			<span class="delta">{data.savingsRate}% retained</span>
		</div>
	</section>
	<section class="grid dashboard-grid">
		<div class="card">
			<div class="card-head">
				<div>
					<span class="eyebrow">Last two weeks</span>
					<h2>Daily spending</h2>
				</div>
			</div>
			<BarsChart data={data.dailySpend} />
		</div>
		<div class="card">
			<div class="card-head">
				<div>
					<span class="eyebrow">Category mix</span>
					<h2>Expense share</h2>
				</div>
			</div>
			<DonutChart data={data.categorySpend} {currency} />
		</div>
		<div class="card span-two">
			<div class="card-head">
				<div>
					<span class="eyebrow">Long view</span>
					<h2>Income against expenses</h2>
				</div>
			</div>
			<TrendChart data={data.monthlyTrend} />
		</div>
	</section>{/if}
