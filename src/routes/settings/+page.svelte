<script lang="ts">
	import { ArchiveRestore, DatabaseBackup, Download, FileUp, ShieldCheck } from '@lucide/svelte';
	import { api } from '$lib/api';
	import CsvWizard from '$lib/components/CsvWizard.svelte';
	import Status from '$lib/components/Status.svelte';
	import { appState } from '$lib/stores/app.svelte';
	import type { Settings, Theme } from '$lib/types/domain';
	let currency = $state('USD'),
		theme = $state<Theme>('light'),
		weekStartsOn = $state<Settings['weekStartsOn']>('monday'),
		saving = $state(false),
		wizard = $state(false),
		toast = $state('');
	$effect(() => {
		if (appState.bootstrap) {
			currency = appState.bootstrap.settings.currency;
			theme = appState.bootstrap.settings.theme;
			weekStartsOn = appState.bootstrap.settings.weekStartsOn;
		}
	});
	async function save() {
		saving = true;
		await appState.saveSettings({ currency, theme, weekStartsOn });
		saving = false;
		notify('Preferences saved');
	}
	function notify(message: string) {
		toast = message;
		setTimeout(() => (toast = ''), 2800);
	}
	async function pathAction(kind: 'export' | 'backup' | 'restore') {
		const fallback = kind === 'export' ? 'fern-transactions.csv' : 'fern-backup.db';
		const path = prompt(
			`Enter a path to ${kind === 'restore' ? 'restore from' : 'save to'}:`,
			fallback
		);
		if (!path) return;
		if (kind === 'export') await api.exportCsv(path);
		if (kind === 'backup') await api.createBackup(path);
		if (kind === 'restore') await api.restoreBackup(path);
		notify(kind === 'restore' ? 'Backup restored' : 'File created successfully');
	}
</script>

<svelte:head><title>Settings · Fern</title></svelte:head>
<header class="page-head">
	<div>
		<span class="eyebrow">Make it yours</span>
		<h1>Settings</h1>
		<p class="muted">Preferences, portability, and peace of mind.</p>
	</div>
</header>
{#if appState.loading}<Status loading />{:else if appState.error}<Status
		error={appState.error}
	/>{:else}<section class="grid settings-grid">
		<div class="card">
			<div class="card-head">
				<div>
					<span class="eyebrow">Preferences</span>
					<h2>Display & formatting</h2>
				</div>
			</div>
			<div class="setting-row">
				<div>
					<h3>Currency</h3>
					<p>Used for all amounts and reports.</p>
				</div>
				<select class="field" bind:value={currency}
					><option>USD</option><option>EUR</option><option>GBP</option><option>CAD</option><option
						>AUD</option
					></select
				>
			</div>
			<div class="setting-row">
				<div>
					<h3>Appearance</h3>
					<p>Choose the light or evening palette.</p>
				</div>
				<select class="field" bind:value={theme}
					><option value="light">Light</option><option value="dark">Dark</option><option
						value="system">System</option
					></select
				>
			</div>
			<div class="setting-row">
				<div>
					<h3>First day of week</h3>
					<p>Applied to future calendar views.</p>
				</div>
				<select class="field" bind:value={weekStartsOn}
					><option value="monday">Monday</option><option value="sunday">Sunday</option></select
				>
			</div>
			<div style="display:flex;justify-content:flex-end;margin-top:18px">
				<button class="button primary" disabled={saving} onclick={save}
					>{saving ? 'Saving…' : 'Save preferences'}</button
				>
			</div>
		</div>
		<aside class="card">
			<div class="card-head">
				<div>
					<span class="eyebrow">Local first</span>
					<h2>Your data stays yours</h2>
				</div>
				<ShieldCheck size={27} color="var(--fern)" />
			</div>
			<p class="muted" style="line-height:1.7">
				Fern’s desktop database lives on your device. Export it whenever you like; no account or
				cloud connection required.
			</p>
		</aside>
		<div class="card">
			<div class="card-head">
				<div>
					<span class="eyebrow">Data tools</span>
					<h2>Import & export</h2>
				</div>
			</div>
			<div class="setting-row">
				<div>
					<h3>Import bank CSV</h3>
					<p>Preview and map columns before anything changes.</p>
				</div>
				<button class="button" onclick={() => (wizard = true)}
					><FileUp size={16} /> Import CSV</button
				>
			</div>
			<div class="setting-row">
				<div>
					<h3>Export transactions</h3>
					<p>Create a portable CSV copy of your ledger.</p>
				</div>
				<button class="button" onclick={() => pathAction('export')}
					><Download size={16} /> Export CSV</button
				>
			</div>
		</div>
		<div class="card danger-zone">
			<div class="card-head">
				<div>
					<span class="eyebrow">Backup</span>
					<h2>Database snapshots</h2>
				</div>
			</div>
			<div class="setting-row">
				<div>
					<h3>Create backup</h3>
					<p>Save a complete, restorable database file.</p>
				</div>
				<button class="button" onclick={() => pathAction('backup')}
					><DatabaseBackup size={16} /> Back up</button
				>
			</div>
			<div class="setting-row">
				<div>
					<h3>Restore backup</h3>
					<p>Replace current data from a Fern backup.</p>
				</div>
				<button class="button danger" onclick={() => pathAction('restore')}
					><ArchiveRestore size={16} /> Restore</button
				>
			</div>
		</div>
	</section>{/if}
{#if wizard}<CsvWizard close={() => (wizard = false)} complete={notify} />{/if}{#if toast}<div
		class="toast"
		role="status"
	>
		{toast}
	</div>{/if}
