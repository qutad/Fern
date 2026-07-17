<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import {
		BarChart3,
		CalendarClock,
		ChartNoAxesCombined,
		CreditCard,
		LayoutDashboard,
		Leaf,
		Settings,
		WalletCards
	} from '@lucide/svelte';
	import { onMount } from 'svelte';
	import { appState } from '$lib/stores/app.svelte';

	let { children } = $props();
	const links = [
		{ href: '/dashboard', label: 'Overview', icon: LayoutDashboard },
		{ href: '/transactions', label: 'Transactions', icon: CreditCard },
		{ href: '/budgets', label: 'Budgets', icon: WalletCards },
		{ href: '/recurring', label: 'Recurring', icon: CalendarClock },
		{ href: '/analytics', label: 'Analytics', icon: ChartNoAxesCombined },
		{ href: '/settings', label: 'Settings', icon: Settings }
	] as const;

	onMount(() => {
		void appState.init();
	});
</script>

<div class="app-shell">
	<aside class="sidebar" aria-label="Main navigation">
		<a class="brand" href={resolve('/dashboard')}
			><span class="brand-mark"><Leaf size={19} /></span> Fern</a
		>
		<nav class="nav">
			{#each links as link (link.href)}
				<a
					href={resolve(link.href)}
					class:active={page.url.pathname === link.href}
					aria-current={page.url.pathname === link.href ? 'page' : undefined}
				>
					<link.icon size={18} strokeWidth={1.8} /> <span>{link.label}</span>
				</a>
			{/each}
		</nav>
		<div class="sidebar-foot">
			<BarChart3 size={16} /><br />Your money, clearly.<br />Private by design.
		</div>
	</aside>
	<main class="main">{@render children()}</main>
</div>
