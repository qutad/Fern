<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import '../app.css';
	import AppShell from '$lib/components/AppShell.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Fern · Personal finance</title>
	<meta name="description" content="A calm, private home for your finances." />
</svelte:head>

<AppShell>{@render children()}</AppShell>
