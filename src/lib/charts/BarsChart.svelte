<script lang="ts">
	type Bar = { date: string; amount: number };
	let { data } = $props<{ data: Bar[] }>();
	let max = $derived(Math.max(...data.map((i: Bar) => i.amount), 1));
</script>

<div
	style="height:210px;display:flex;align-items:flex-end;gap:clamp(4px,1vw,12px);padding-top:15px;border-bottom:1px solid var(--line)"
	role="img"
	aria-label="Daily spending bars"
>
	{#each data as item, i (item.date)}<div
			title={`${item.date}: ${item.amount}`}
			style="height:100%;flex:1;display:flex;align-items:flex-end"
		>
			<div
				style={`width:100%;height:${(item.amount / max) * 90}%;min-height:3px;border-radius:5px 5px 1px 1px;background:${i === data.length - 1 ? 'var(--coral)' : 'var(--fern-soft)'}`}
			></div>
		</div>{/each}
</div>
