<script lang="ts">
	type Trend = { month: string; income: number; expenses: number };
	let { data } = $props<{ data: Trend[] }>();
	let width = 600,
		height = 190,
		pad = 22;
	let max = $derived(Math.max(...data.flatMap((d: Trend) => [d.income, d.expenses]), 1));
	function points(key: 'income' | 'expenses') {
		return data
			.map(
				(d: Trend, i: number) =>
					`${pad + i * ((width - pad * 2) / Math.max(data.length - 1, 1))},${height - pad - (d[key] / max) * (height - pad * 2)}`
			)
			.join(' ');
	}
</script>

<div class="chart-box">
	<svg
		viewBox={`0 0 ${width} ${height + 25}`}
		width="100%"
		height="100%"
		role="img"
		aria-label="Income and expense trend"
	>
		{#each [0, 1, 2, 3] as line (line)}<line
				x1={pad}
				y1={pad + line * 45}
				x2={width - pad}
				y2={pad + line * 45}
				stroke="var(--line)"
				stroke-dasharray="3 5"
			/>{/each}
		<polyline
			points={points('income')}
			fill="none"
			stroke="#557d60"
			stroke-width="3"
			stroke-linecap="round"
			stroke-linejoin="round"
		/>
		<polyline
			points={points('expenses')}
			fill="none"
			stroke="#d96548"
			stroke-width="3"
			stroke-linecap="round"
			stroke-linejoin="round"
		/>
		{#each data as item, i (item.month)}<text
				x={pad + i * ((width - pad * 2) / Math.max(data.length - 1, 1))}
				y={height + 12}
				text-anchor="middle"
				fill="var(--muted)"
				font-size="11">{item.month}</text
			>{/each}
	</svg>
</div>
