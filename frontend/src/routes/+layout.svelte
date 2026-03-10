<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { page } from '$app/state';

	let { children } = $props();

	const navItems = [
		{ path: '/', label: '~' },
		{ path: '/projects', label: 'projects' },
		{ path: '/skills', label: 'skills' },
		{ path: '/blog', label: 'blog' },
		{ path: '/contact', label: 'contact' }
	];
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="layout">
	<header>
		<nav>
			<span class="nav-prefix">kevin@portfolio</span>
			<span class="nav-sep">:</span>
			{#each navItems as item (item.path)}
				<a href={item.path} class="nav-item" class:active={page.url.pathname === item.path}>
					{item.path === '/' ? '~' : item.label}
				</a>
			{/each}
			<span class="nav-suffix">$</span>
		</nav>
	</header>

	<main>
		{@render children()}
	</main>

	<footer>
		<span>// {new Date().getFullYear()} · kevin@portfolio · built with rust + sveltekit</span>
	</footer>
</div>

<style>
	.layout {
		max-width: 860px;
		margin: 0 auto;
		padding: 0 1.5rem;
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}

	header {
		position: sticky;
		top: 0;
		background: rgba(12, 12, 12, 0.95);
		backdrop-filter: blur(8px);
		border-bottom: 1px solid var(--border-subtle);
		z-index: 100;
		padding: 0.75rem 0;
	}

	nav {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.8rem;
		flex-wrap: wrap;
	}

	.nav-prefix {
		color: var(--accent);
	}

	.nav-sep {
		color: var(--text-dim);
		margin-right: 0.5rem;
	}

	.nav-item {
		color: var(--text-muted);
		padding: 0.2rem 0.6rem;
		border: 1px solid transparent;
		border-radius: 2px;
		transition: all 0.15s;
		text-decoration: none;
	}

	.nav-item:hover {
		color: var(--text);
		border-color: var(--border);
		text-decoration: none;
	}

	.nav-item.active {
		color: var(--text);
		border-color: var(--border);
		background: var(--surface);
	}

	.nav-suffix {
		color: var(--text-dim);
		margin-left: 0.25rem;
	}

	main {
		flex: 1;
		padding: 2.5rem 0;
	}

	footer {
		padding: 1.5rem 0;
		border-top: 1px solid var(--border-subtle);
		font-size: 0.7rem;
		color: var(--text-dim);
	}
</style>
