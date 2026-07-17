<script lang="ts">
	import { api } from '$lib/api';
	import type { CsvPreview } from '$lib/types/domain';
	import Modal from './Modal.svelte';
	let { close, complete } = $props<{ close: () => void; complete: (message: string) => void }>();
	let step = $state(1),
		path = $state(''),
		preview = $state<CsvPreview | null>(null),
		busy = $state(false),
		error = $state('');
	let date = $state(''),
		description = $state(''),
		amount = $state(''),
		category = $state(''),
		type = $state('');
	async function inspect() {
		if (!path) return;
		busy = true;
		error = '';
		try {
			preview = await api.previewCsv(path);
			date = preview.headers.find((h) => /date/i.test(h)) ?? '';
			description = preview.headers.find((h) => /desc|merchant|name/i.test(h)) ?? '';
			amount = preview.headers.find((h) => /amount|value/i.test(h)) ?? '';
			category = preview.headers.find((h) => /category/i.test(h)) ?? '';
			type = preview.headers.find((h) => /type/i.test(h)) ?? '';
			step = 2;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Could not preview CSV';
		} finally {
			busy = false;
		}
	}
	async function run() {
		busy = true;
		try {
			const result = await api.importCsv(path, {
				date,
				description,
				amount,
				category: category || undefined,
				type: type || undefined
			});
			step = 3;
			complete(`Imported ${result.imported} transactions`);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Import failed';
		} finally {
			busy = false;
		}
	}
</script>

<Modal title="Import transactions" {close}
	><div class="wizard-steps">
		{#each [1, 2, 3] as n (n)}<i class:active={n <= step}></i>{/each}
	</div>
	{#if step === 1}<span class="eyebrow">Step 1 · Choose a file</span>
		<p class="muted">
			Enter the CSV path. In the desktop app this is read locally and never uploaded.
		</p>
		<label
			>CSV file path<input
				class="field"
				bind:value={path}
				placeholder="/home/you/Downloads/transactions.csv"
			/></label
		>
	{:else if step === 2 && preview}<span class="eyebrow">Step 2 · Map columns</span>
		<p class="muted">Match your bank’s headings to Fern fields. {preview.totalRows} rows found.</p>
		<div class="form-grid">
			{#each [['Date', date], ['Description', description], ['Amount', amount], ['Category', category], ['Type', type]] as field, i (field[0])}<label
					>{field[0]}{i > 2 ? ' (optional)' : ''}<select
						class="field"
						value={field[1]}
						onchange={(e) => {
							const v = e.currentTarget.value;
							if (i === 0) date = v;
							if (i === 1) description = v;
							if (i === 2) amount = v;
							if (i === 3) category = v;
							if (i === 4) type = v;
						}}
						><option value="">Not mapped</option
						>{#each preview.headers as h, i (`${h}-${i}`)}<option value={h}>{h}</option
							>{/each}</select
					></label
				>{/each}
		</div>
		<div class="preview" style="margin-top:18px">
			<table>
				<thead
					><tr
						>{#each preview.headers as h, i (`${h}-${i}`)}<th>{h}</th>{/each}</tr
					></thead
				><tbody
					>{#each preview.rows as row, rowIndex (`${row.join('\u0000')}-${rowIndex}`)}<tr
							>{#each row as cell, cellIndex (`${preview.headers[cellIndex] ?? cellIndex}-${cellIndex}`)}<td
									>{cell}</td
								>{/each}</tr
						>{/each}</tbody
				>
			</table>
		</div>
	{:else}<div class="empty">
			<div>
				<div class="serif" style="font-size:2rem">Ready.</div>
				<p>Your imported activity is now in Fern.</p>
			</div>
		</div>{/if}
	{#if error}<p class="error" role="alert">{error}</p>{/if}
	<div class="modal-actions">
		<button class="button" onclick={close}>{step === 3 ? 'Close' : 'Cancel'}</button
		>{#if step === 1}<button class="button primary" disabled={!path || busy} onclick={inspect}
				>{busy ? 'Reading…' : 'Preview columns'}</button
			>{:else if step === 2}<button
				class="button primary"
				disabled={!date || !description || !amount || busy}
				onclick={run}>{busy ? 'Importing…' : 'Import transactions'}</button
			>{/if}
	</div></Modal
>
