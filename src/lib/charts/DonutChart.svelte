<script lang="ts">
	import { formatMoney } from '$lib/utils/format';
	type Slice = { category: string; amount: number; color: string };
	let { data, currency = 'USD' } = $props<{ data: Slice[]; currency?: string }>();
	let total = $derived(data.reduce((sum: number, item: Slice) => sum + item.amount, 0));
	let gradient = $derived.by(() => {
		let at = 0;
		return data
			.map((item: Slice) => {
				const start = at;
				at += total ? (item.amount / total) * 100 : 0;
				return `${item.color} ${start}% ${at}%`;
			})
			.join(', ');
	});
</script>

<div style="display:grid;place-items:center;gap:22px">
	<div
		style={`width:160px;height:160px;border-radius:50%;display:grid;place-items:center;background:conic-gradient(${gradient || 'var(--line) 0 100%'})`}
	>
		<div
			style="width:104px;height:104px;border-radius:50%;display:grid;place-items:center;text-align:center;background:var(--paper)"
		>
			<div>
				<small class="muted">Total spent</small><br /><strong
					>{formatMoney(total, currency, true)}</strong
				>
			</div>
		</div>
	</div>
	<div class="legend">
		{#each data.slice(0, 5) as item (item.category)}<span
				><i style={`background:${item.color}`}></i>{item.category}</span
			>{/each}
	</div>
</div>
