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
		Menu,
		MoreHorizontal,
		Settings,
		WalletCards,
		X
	} from '@lucide/svelte';
	import { onMount } from 'svelte';
	import { appState } from '$lib/stores/app.svelte';

	let { children } = $props();
	let moreOpen = $state(false);
	const links = [
		{ href: '/dashboard', label: 'Overview', icon: LayoutDashboard },
		{ href: '/transactions', label: 'Transactions', icon: CreditCard },
		{ href: '/budgets', label: 'Budgets', icon: WalletCards },
		{ href: '/recurring', label: 'Recurring', icon: CalendarClock },
		{ href: '/analytics', label: 'Analytics', icon: ChartNoAxesCombined },
		{ href: '/settings', label: 'Settings', icon: Settings }
	] as const;
	const mobileLinks = links.filter((link) =>
		['/dashboard', '/transactions', '/budgets', '/analytics'].includes(link.href)
	);
	const secondaryLinks = links.filter((link) => ['/recurring', '/settings'].includes(link.href));
	const isActive = (href: string) =>
		(page.url.pathname === '/' && href === '/dashboard') || page.url.pathname.startsWith(href);

	onMount(() => {
		void appState.init();
	});
</script>

<div class="app-shell">
	<header class="mobile-header">
		<a class="brand mobile-brand" href={resolve('/dashboard')}
			><span class="brand-mark"><Leaf size={18} /></span> Fern</a
		>
		<button class="mobile-menu-button" aria-label="Open menu" onclick={() => (moreOpen = true)}>
			<Menu size={25} />
		</button>
	</header>
	<aside class="sidebar" aria-label="Main navigation">
		<a class="brand" href={resolve('/dashboard')}
			><span class="brand-mark"><Leaf size={19} /></span> Fern</a
		>
		<nav class="nav">
			{#each links as link (link.href)}
				<a
					href={resolve(link.href)}
					class:active={isActive(link.href)}
					aria-current={isActive(link.href) ? 'page' : undefined}
				>
					<link.icon size={18} strokeWidth={1.8} /> <span>{link.label}</span>
				</a>
			{/each}
		</nav>
		<div class="sidebar-foot">
			<BarChart3 size={16} /><br />Your money, clearly.<br />Private by design.
		</div>
	</aside>
	<nav class="mobile-nav" aria-label="Mobile navigation">
		{#each mobileLinks as link (link.href)}
			<a
				href={resolve(link.href)}
				class:active={isActive(link.href)}
				aria-current={isActive(link.href) ? 'page' : undefined}
			>
				<link.icon size={23} strokeWidth={1.8} /> <span>{link.label}</span>
			</a>
		{/each}
		<button
			class:active={secondaryLinks.some((link) => isActive(link.href))}
			aria-expanded={moreOpen}
			onclick={() => (moreOpen = true)}
		>
			<MoreHorizontal size={25} strokeWidth={2} /><span>More</span>
		</button>
	</nav>
	<main class="main">{@render children()}</main>
</div>

{#if moreOpen}
	<button class="mobile-menu-backdrop" aria-label="Close menu" onclick={() => (moreOpen = false)}
	></button>
	<aside class="mobile-menu" aria-label="More navigation">
		<div class="mobile-menu-head">
			<div>
				<span class="eyebrow">Navigate</span>
				<h2>More</h2>
			</div>
			<button class="icon-btn" aria-label="Close menu" onclick={() => (moreOpen = false)}
				><X /></button
			>
		</div>
		{#each secondaryLinks as link (link.href)}
			<a
				href={resolve(link.href)}
				class:active={isActive(link.href)}
				onclick={() => (moreOpen = false)}
			>
				<span><link.icon size={21} /></span>
				<div>
					<strong>{link.label}</strong><small
						>{link.href === '/recurring'
							? 'Bills and repeating income'
							: 'Preferences and data tools'}</small
					>
				</div>
			</a>
		{/each}
	</aside>
{/if}
